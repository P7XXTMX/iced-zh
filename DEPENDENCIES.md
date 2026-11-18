# 依赖

Iced 需要一些系统依赖才能工作，而并非所有操作系统
都预装了它们。

你可以按照下面针对你的系统的说明来获取这些依赖；
如果这里没有你的系统，欢迎补充！

## NixOS

你可以把这个 `shell.nix` 添加到项目中，并通过运行 `nix-shell` 来使用它：

```nix
{ pkgs ? import <nixpkgs> { } }:

let
  dlopenLibraries = with pkgs; [
    libxkbcommon

    # GPU 后端
    vulkan-loader
    # libGL

    # 窗口系统
    wayland
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
  ];
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
  ];

  # 你的项目在构建时链接的其他库，
  # 例如 OpenSSL
  buildInputs = [];

  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";
}
```

或者，你也可以用这个 `flake.nix` 创建一个开发 shell，通过 `nix develop` 激活：

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = { nixpkgs, systems, ... }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = nixpkgs.legacyPackages;
    in {
      devShells = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
          dlopenLibraries = with pkgs; [
            libxkbcommon

            # GPU 后端
            vulkan-loader
            # libGL

            # 窗口系统
            wayland
            # xorg.libX11
            # xorg.libXcursor
            # xorg.libXi
          ];
        in {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              rustc
            ];

            # 你的项目在构建时链接的其他库，
            # 例如 OpenSSL
            buildInputs = [];

            env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${nixpkgs.lib.makeLibraryPath dlopenLibraries}";
          };
        });
    };
}
```

<details>
<summary>显示英文原文</summary>

# Dependencies

Iced requires some system dependencies to work, and not
all operating systems come with them installed.

You can follow the provided instructions for your system to
get them, if your system isn't here, add it!

## NixOS

You can add this `shell.nix` to your project and use it by running `nix-shell`:

```nix
{ pkgs ? import <nixpkgs> { } }:

let
  dlopenLibraries = with pkgs; [
    libxkbcommon

    # GPU backend
    vulkan-loader
    # libGL

    # Window system
    wayland
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
  ];
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
  ];

  # additional libraries that your project
  # links to at build time, e.g. OpenSSL
  buildInputs = [];

  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";
}
```

Alternatively, you can use this `flake.nix` to create a dev shell, activated by `nix develop`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = { nixpkgs, systems, ... }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = nixpkgs.legacyPackages;
    in {
      devShells = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
          dlopenLibraries = with pkgs; [
            libxkbcommon

            # GPU backend
            vulkan-loader
            # libGL

            # Window system
            wayland
            # xorg.libX11
            # xorg.libXcursor
            # xorg.libXi
          ];
        in {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              rustc
            ];

            # additional libraries that your project
            # links to at build time, e.g. OpenSSL
            buildInputs = [];

            env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${nixpkgs.lib.makeLibraryPath dlopenLibraries}";
          };
        });
    };
}
```

</details>
