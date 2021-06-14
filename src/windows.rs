fn install() {
    let tmpdir = "C:/Windows/Temp";
    fileget(pkgurl, tmpdir);
    let pkgfile = pkgurl.split("/").last().unwrap();
    targz_unpack(tmpdir + "/" + pkgfile);
    ruby! {
        def cp(src, dest, preserve: nil, noop: nil, verbose: nil)
            fu_output_message "cp#{preserve ? ' -p' : ''} #{[src,dest].flatten.join ' '}" if verbose
            return if noop
            fu_each_src_dest(src, dest) do |s, d|
                copy_file s, d, preserve
            end
        end
        
        cp(tmpdir + "/src/*", "C:/Windows/Fonts/*")
    }
}