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
        # Exceptions would fail to propagate across the app boundary and just complicate our code.
        # It's better to just abort in case of a catastrophic failure.
        '-fno-exceptions',
        # RTTI is not useful to us.
        '-fno-rtti',
        # Hide all symbols from this library, unless they are exported explicitly.
        '-fvisibility=hidden', '-fvisibility-inlines-hidden',
        language : 'cpp',
    )

    add_global_link_arguments(
        # This fixes the way stdcall functions get exported by the linker.
        '-Wl,--add-stdcall-alias',
        # Enable auto importing symbols as needed
        '-Wl,--enable-auto-import',
        language : 'cpp',
    )

    # Since we have many stub methods, we ignore a lot of parameters.
    add_global_arguments('-Wno-unused-parameter', language : 'cpp')

    if compiler_id == 'gcc'
        # GCC will complain about the COM interfaces, but we can't modify them.
        add_global_arguments('-Wno-non-virtual-dtor', language : 'cpp')
    endif
endif

subdir('src')
