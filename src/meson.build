srcs = [
    'core/entrypoint.cpp',
    'core/core.cpp',

    'util/log.cpp',
    'util/str.cpp',
]

d3d9 = shared_library(
    'd3d9',
    sources : srcs,
    dependencies : [ dxgi_dep ],
    cpp_pch : 'pch/common.hpp',
    name_prefix : '',
    name_suffix : 'dll',
)
