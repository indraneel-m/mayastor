{ lib
, stdenv
, fetchFromGitHub
, makeWrapper
, libaio
, python3
, zlib
, withGnuplot ? false
, gnuplot ? null
}:

stdenv.mkDerivation rec {
  pname = "fio";
  version = "3.30";

  src = fetchFromGitHub {
    owner = "axboe";
    repo = "fio";
    rev = "fio-${version}";
    sha256 = "sha256-X2B8xlCLSHDgTaH55TUJq4WcaObZy/fvfkQ0j3J9Kzo=";
  };

  buildInputs = [ python3 zlib ]
    ++ lib.optional (!stdenv.isDarwin) libaio;

  nativeBuildInputs = [ makeWrapper ];

  strictDeps = true;

  enableParallelBuilding = true;

  postPatch = ''
    substituteInPlace Makefile \
      --replace "mandir = /usr/share/man" "mandir = \$(prefix)/man" \
      --replace "sharedir = /usr/share/fio" "sharedir = \$(prefix)/share/fio"
    substituteInPlace tools/plot/fio2gnuplot --replace /usr/share/fio $out/share/fio
  '';

  preInstall = ''
    mkdir -p $dev/include
    cp -p --parents $(find . -name "*.h") $dev/include
  '';

  postInstall = lib.optionalString withGnuplot ''
    wrapProgram $out/bin/fio2gnuplot \
      --prefix PATH : ${lib.makeBinPath [ gnuplot ]}
  '';

  outputs = [ "out" "dev" ];
  setOutputFlags = false;

  meta = with lib; {
    description = "Flexible IO Tester - an IO benchmark tool";
    homepage = "https://git.kernel.dk/cgit/fio/";
    license = licenses.gpl2;
    platforms = platforms.unix;
  };
}
