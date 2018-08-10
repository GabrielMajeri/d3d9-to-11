#pragma once

// This file include the big Windows API headers.

// This define gets rid of most of the clutter.
#define WIN32_LEAN_AND_MEAN

// These defines disable some more useless includes.
#define NOMINMAX
#define NOATOM
#define NOGDICAPMASKS
#define NOMETAFILE
#define NOMINMAX
#define NOOPENFILE
#define NORASTEROPS
#define NOSCROLL
#define NOSOUND
#define NOSYSMETRICS
#define NOTEXTMETRIC
#define NOWH
#define NOCOMM
#define NOKANJI
#define NOCRYPT
#define NOMCX

// Include the massive beast of a header.
#include <windows.h>
