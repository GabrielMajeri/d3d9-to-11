srcs = [
    'core/entrypoint.cpp',
    'core/core.cpp',

    'util/log.cpp',
]

d3d9 = shared_library(
    'd3d9',
    sources : srcs,
    cpp_pch : 'pch/common.hpp',
    name_prefix : '',
    name_suffix : 'dll',
)
