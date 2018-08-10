project(
    'd3d9-to-11',
    ['cpp'],
    license : 'LGPL3+',
    meson_version : '>0.47',
    version : '0.1.0',
    default_options : [
        'cpp_std=c++17',
        'warning_level=3',
        'werror=true',
    ],
)

compiler = meson.get_compiler('cpp')

compiler_id = compiler.get_id()

if compiler_id == 'gcc' or compiler_id == 'clang'
    add_global_arguments(
        # Hide all symbols from this library, unless they are exported explicitly.
        '-fvisibility=hidden', '-fvisibility-inlines-hidden',
        language : 'cpp',
    )

    add_global_link_arguments(
        # We link against the standard library statically,
        # to simplify distribution and development.
        '-static',
        '-static-libstdc++',
        # Fix link issues by including pthreads
        '-pthread', '-Wl,--whole-archive', '-lpthread', '-Wl,--no-whole-archive',
        # This fixes the way stdcall functions get exported by the linker.
        '-Wl,--add-stdcall-alias',
        language : 'cpp',
    )
endif

subdir('src')
