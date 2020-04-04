// (glob, lang)
// extensions should be splitted by ; and compared individually
/*
pub static LANG_TYPES: &[(&str, &str)] = &[
    ("rs", "rust"),
    ("java", "java"),
    ("r", "r"),
    ("c", "c"),
    ("cpp", "cpp"),
    ("go", "go"),
    ("html", "html"),
    ("json", "json"),
    ("jl", "julia"),
    ("tex", "latex"),
    ("lua", "lua"),
    ("m", "matlab"),
    ("php", "php"),
    ("py", "python"),
    ("rb", "ruby"),
    ("toml", "toml"),
    ("xml", "xml"),
    ("md", "markdown"),
    ("ron", "markdown"),
];
*/

pub static LANG_TYPES: &[(&str, &str)] = &[
("*.abnf", "abnf"),
("*.as", "actionscript"),
("*.adb;*.ads", "ada"),
("*.4th;*.forth", "ansforth94"),
("*.asp", "asp"),
("Makefile.am;GNUmakefile.am", "automake"),
("*.awk", "awk"),
("*.prg", "bennugd"),
("*.bib", "bibtex"),
("*.bsv", "bluespec"),
("*.boo", "boo"),
("*.cg", "cg"),
("ChangeLog*", "changelog"),
("*.h", "chdr"),
("*.c", "c"),
("CMakeLists.txt;*.cmake;*.cmake.in;*.ctest;*.ctest.in", "cmake"),
("*.cbl;*.cob;*.cbd;*.cdb;*.cdc", "cobol"),
("*.hh;*.hp;*.hpp;*.h++", "cpphdr"),
("*.cpp;*.cxx;*.cc;*.C;*.c++;*.tpp", "cpp"),
("*.cs", "c-sharp"),
("*.css;*.CSSL", "css"),
("*.csv", "csv"),
("*.cu;*.cuh", "cuda"),
("*.def","def"),
("*.desktop;*.kdelnk", "desktop"),
("*.diff;*.patch;*.rej", "diff"),
("*.d", "d"),
("*.docbook", "docbook"),
("*.bat;*.cmd;*.sys", "dosbatch"),
("*.dot;*.gv", "dot"),
("*.dpatch", "dpatch"),
("*.dtd", "dtd"),
("*.dtl", "dtl"),
("*.e;*.eif", "eiffel"),
("*.erl;*.hrl", "erlang"),
("*.fcl", "fcl"),
("*.frt;*.fs", "forth"),
("*.f;*.f90;*.f95;*.for;*.F;*.F90", "fortran"),
("*.fs;", "fsharp"),
("*.g;*.gd;*.gi;*.gap", "gap"),
("*.gdb", "gdb-log"),
("*.gs", "genie"),
("*.glslv;*.glslf;*.glsl", "glsl"),
("*.go", "go"),
("*.groovy", "groovy"),
("*.gtk-doc","gtk-doc"),
("gtkrc;.gtkrc;gtkrc-*;.gtkrc-*", "gtkrc"),
("*.haddock","haddock"),
("*.hs", "haskell"),
("*.lhs", "haskell-literate"),
("*.hx", "haxe"),
("*.html;*.htm", "html"),
("*.pro", "idl-exelis"),
("*.idl", "idl"),
("*.ijm", "imagej"),
("*.ini", "ini"),
("*.jade;*.pug", "jade"),
("*.java", "java"),
("*.js;*.node", "js"),
("*.ijs", "j"),
("*.json;*.geojson;*.topojson", "json"),
("*.jl", "julia"),
("*.kt", "kotlin"),
("*.tex;*.ltx;*.sty;*.cls;*.dtx;*.ins;*.bbl", "latex"),
("*.less", "less"),
("*.l;*.lex;*.flex", "lex"),
("*.la;*.lai;*.lo", "libtool"),
("*.ll", "llvm"),
("*.logcat", "logcat"),
("*.lgt", "logtalk"),
("*.lua", "lua"),
("*.m4;configure.ac;configure.in", "m4"),
("[Mm]akefile;GNUmakefile;*.make;*.mak;*.mk", "makefile"),
("*.page", "mallard"),
("*.markdown;*.md;*.mkd", "markdown"),
("*.m", "matlab"),
("*.mac;*.MAC;*.dem;*.DEM;*.wxm;*.WXM", "maxima"),
("*.mediawiki","mediawiki"),
("meson.build;meson_options.txt", "meson"),
("*.mo;*.mop", "modelica"),
("*.mxml", "mxml"),
("*.n", "nemerle"),
("*.nrx", "netrexx"),
("*.nsi;*.nsh", "nsis"),
("*.m", "objc"),
("*.j", "objj"),
("*.ml;*.mli;*.mll;*.mly", "ocaml"),
("*.ocl", "ocl"),
("*.m", "octave"),
("*.ooc", "ooc"),
("*.sign;*.impl", "opal"),
("*.cl", "opencl"),
("*.p;*.pas", "pascal"),
("*.pl;*.pm;*.al;*.perl;*.t", "perl"),
("*.php;*.php3;*.php4;*.phtml", "php"),
("*.pig", "pig"),
("*.pc", "pkgconfig"),
("*.po;*.pot", "gettext-translation"),
("*.ps1;*.psm1;*.psd1", "powershell"),
("*.prolog", "prolog"),
("*.proto", "proto"),
("*.pp", "puppet"),
("*.py3", "python3"),
("*.py", "python"),
("*.R;*.Rout;*.r;*.Rhistory;*.Rt;*.Rout.save;*.Rout.fail", "r"),
("*.spec", "rpmspec"),
("*.rst", "rst"),
("*.rb;*.rake;*.gemspec;Rakefile;Capfile;Gemfile", "ruby"),
("*.rs", "rust"),
("*.scala", "scala"),
("*.scm", "scheme"),
("*.sce;*.sci", "scilab"),
("*.scss", "scss"),
("*.sh;*bashrc;.profile;.bash_profile", "sh"),
("*.sml;*.sig", "sml"),
("*.rq", "sparql"),
("*.sql", "sql"),
("*.rnw;*.Rnw;*.snw;*.Snw", "sweave"),
("*.swift", "swift"),
("*.sv;*.svh", "systemverilog"),
("*.t2t", "t2t"),
("*.tcl;*.tk", "tcl"),
("*.tera", "tera"),
("*.texi;*.texinfo", "texinfo"),
("*.thrift", "thrift"),
("*.toml;*.tml;*.lock", "toml"),
("*.vala;*.vapi", "vala"),
("*.vb", "vbnet"),
("*.v", "verilog"),
("*.vhd", "vhdl"),
("*.xml;*.xspf;*.siv;*.smil;*.smi;*.sml;*.kino;*.xul;*.xbel;*.abw;*.zabw;*.glade;*.jnlp;*.xhtml;*.svg;*.mml;*.rdf;*.rss;*.wml;*.xmi;*.fo;*.xslfo;*.sgml", "xml"),
("*.xslt;*.xsl", "xslt"),
("*.y;*.yacc", "yacc"),
("*.yaml;*.yml;", "yaml"),
    ];
// returns the lang directly by the glob
pub fn get_by_left(glob: &str) -> Option<&str> {
    for i in LANG_TYPES {
        let globs = glob.split(";");
        for k in globs {
            if glob == k {
                // Return on match
                return Some(i.1);
            }
        }
    }
    None
}

pub fn file_extension_to_lang(extension: &str) -> Option<&str> {
    // First convert to glob
    for i in LANG_TYPES {
        let globs = i.0.split(";");
        for k in globs {
            if format!("*.{}", extension) == k.to_string() {
                return Some(i.1);
            }
        }
    }
    // Not found
    Some("markdown")
}

pub fn file_extension_to_glob(extension: &str) -> String {
    // "rs" -> "*.rs"
    // Returning the glob
    for i in LANG_TYPES {
        let globs = i.0.split(";");
        for k in globs {
            if k.to_string().contains(extension) {
                return k.to_string();
            }
        }
    }
    "*.markdown".to_string()
}

// returns the array of possible globs based on the language
pub fn get_by_right(lang: &str) -> Vec<Option<&str>> {
    for j in LANG_TYPES {
        if lang == j.1 {
            let mut globs = Vec::<Option<&str>>::new();
            let stored_globs = j.0.split(";");
            for elem in stored_globs {
                globs.push(Some(elem));
            }
            return globs;
        }
    }
    vec![None]
}
