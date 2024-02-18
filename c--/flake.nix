{
  description = "Collection of c/c++ pearls";

  outputs = { self, nixpkgs }: {

    packages.x86_64-linux.cpp_pearls = nixpkgs.legacyPackages.x86_64-linux.cpp_pearls;

    packages.x86_64-linux.default = self.packages.x86_64-linux.cpp_pearls;

  };
}
