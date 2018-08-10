# History of the project

What follows is a brief timeline of this project's development.

## Creation

I initially started work on my idea by forking DXVK and building on top of its infrastructure.

After a [discussion in this pull request](https://github.com/doitsujin/dxvk/pull/541),
I've realised it's better to keep the projects separate.

DXVK will focus on giving us the best D3D11 support,
while this D3D9-to-D3D11 wrapper will focus only on the conversion between the two APIs.
