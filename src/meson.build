srcs = [
    'core/entrypoint.cpp',
    'core/core.cpp',
    'core/adapter.cpp',

    'util/log.cpp',
    'util/str.cpp',
]

d3d9 = shared_library(
    'd3d9',
    sources : srcs,
    dependencies : [ d3d11_dep, dxgi_dep ],
    cpp_pch : 'pch/common.hpp',
    name_prefix : '',
    name_suffix : 'dll',
)
