import os
import os.path
import subprocess
import sys


# Flags from YCM's default config.
flags = [
'-DUSE_CLANG_COMPLETER',
'-std=c++14',
'-I',
'/usr/include/boost/',
'-I',
'/mnt/d/Personal/chromium/src/v8',
'-I',
'/mnt/d/Personal/chromium/src/v8/src',
'-I',
'/mnt/d/Personal/chromium/src/v8/include',
'-I',
'/mnt/c/Program Files (x86)/Microsoft Visual Studio/2017/Professional/VC/Tools/MSVC/14.16.27023/include',
'-I',
'src',
'-x',
'c++',
]

# Flags from YCM's default config.
flags_macos = [
'-DUSE_CLANG_COMPLETER',
'-std=gnu++14',
'-I',
'/usr/local/Cellar/boost/1.67.0_1',
'-I',
'/Users/zuoxinyu/PersonalProjects/v8',
'-I',
'/Users/zuoxinyu/PersonalProjects/v8/include',
'-x',
'c++',
]


def PathExists(*args):
  return os.path.exists(os.path.join(*args))


def FindV8SrcFromFilename(filename):
  """Searches for the root of the V8 checkout.

  Simply checks parent directories until it finds .gclient and v8/.

  Args:
    filename: (String) Path to source file being edited.

  Returns:
    (String) Path of 'v8/', or None if unable to find.
  """
  curdir = os.path.normpath(os.path.dirname(filename))
  while not (PathExists(curdir, 'v8') and PathExists(curdir, 'v8', 'DEPS')
             and (PathExists(curdir, '.gclient')
                  or PathExists(curdir, 'v8', '.git'))):
    nextdir = os.path.normpath(os.path.join(curdir, '..'))
    if nextdir == curdir:
      return None
    curdir = nextdir
  return os.path.join(curdir, 'v8')


def GetClangCommandFromNinjaForFilename(v8_root, filename):
  """Returns the command line to build |filename|.

  Asks ninja how it would build the source file. If the specified file is a
  header, tries to find its companion source file first.

  Args:
    v8_root: (String) Path to v8/.
    filename: (String) Path to source file being edited.

  Returns:
    (List of Strings) Command line arguments for clang.
  """
  if not v8_root:
    return []

  # Generally, everyone benefits from including V8's root, because all of
  # V8's includes are relative to that.
  v8_flags = ['-I' + os.path.join(v8_root)]

  # Version of Clang used to compile V8 can be newer then version of
  # libclang that YCM uses for completion. So it's possible that YCM's libclang
  # doesn't know about some used warning options, which causes compilation
  # warnings (and errors, because of '-Werror');
  v8_flags.append('-Wno-unknown-warning-option')

  # Header files can't be built. Instead, try to match a header file to its
  # corresponding source file.
  if filename.endswith('.h'):
    alternates = ['.cc', '.cpp']
    for alt_extension in alternates:
      alt_name = filename[:-2] + alt_extension
      if os.path.exists(alt_name):
        filename = alt_name
        break
    else:
      if filename.endswith('-inl.h'):
        for alt_extension in alternates:
          alt_name = filename[:-6] + alt_extension
          if os.path.exists(alt_name):
            filename = alt_name
            break;
        else:
          # If this is a standalone -inl.h file with no source, the best we can
          # do is try to use the default flags.
          return v8_flags
      else:
        # If this is a standalone .h file with no source, the best we can do is
        # try to use the default flags.
        return v8_flags

  sys.path.append(os.path.join(v8_root, 'tools', 'ninja'))
  from ninja_output import GetNinjaOutputDirectory
  out_dir = os.path.realpath(GetNinjaOutputDirectory(v8_root))

  # Ninja needs the path to the source file relative to the output build
  # directory.
  rel_filename = os.path.relpath(os.path.realpath(filename), out_dir)

  # Ask ninja how it would build our source file.
  p = subprocess.Popen(['ninja', '-v', '-C', out_dir, '-t',
                        'commands', rel_filename + '^'],
                       stdout=subprocess.PIPE)
  stdout, stderr = p.communicate()
  if p.returncode:
    return v8_flags

  # Ninja might execute several commands to build something. We want the last
  # clang command.
  clang_line = None
  for line in reversed(stdout.split('\n')):
    if 'clang' in line:
      clang_line = line
      break
  else:
    return v8_flags

  # Parse flags that are important for YCM's purposes.
  for flag in clang_line.split(' '):
    if flag.startswith('-I'):
      # Relative paths need to be resolved, because they're relative to the
      # output dir, not the source.
      if flag[2] == '/':
        v8_flags.append(flag)
      else:
        abs_path = os.path.normpath(os.path.join(out_dir, flag[2:]))
        v8_flags.append('-I' + abs_path)
    elif flag.startswith('-std'):
      v8_flags.append(flag)
    elif flag.startswith('-') and flag[1] in 'DWFfmO':
      if flag == '-Wno-deprecated-register' or flag == '-Wno-header-guard':
        # These flags causes libclang (3.3) to crash. Remove it until things
        # are fixed.
        continue
      v8_flags.append(flag)

  return v8_flags


def FlagsForFile(filename):
  """This is the main entry point for YCM. Its interface is fixed.

  Args:
    filename: (String) Path to source file being edited.

  Returns:
    (Dictionary)
      'flags': (List of Strings) Command line flags.
      'do_cache': (Boolean) True if the result should be cached.
  """
  v8_root = FindV8SrcFromFilename(filename)
  v8_flags = GetClangCommandFromNinjaForFilename(v8_root, filename)
  final_flags = flags + v8_flags
  return {
    'flags': final_flags,
    'do_cache': True
  }
