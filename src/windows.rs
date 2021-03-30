fn install() {
    let tmpdir = "C:/Windows/Temp";
    fileget(pkgurl, tmpdir);
    let pkgfile = pkgurl.split("/").last().unwrap();
    targz_unpack(tmpdir + "/" + pkgfile);
    cp(tmpdir + "/src/*", "C:/Windows/Fonts/*")
}