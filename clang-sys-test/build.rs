extern crate ctest;

fn llvm_config(path: &str, arg: &str) -> String {
    let stdout = std::process::Command::new(&path).arg(arg).output()
        .expect("failed to execute process").stdout;
    String::from_utf8(stdout).unwrap().split_whitespace().next().unwrap().to_string()
}

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    let llvm_config_path = std::env::var("LLVM_CONFIG_PATH").unwrap_or_else(|_| "llvm-config".to_string());
    let llvm_include_dir = llvm_config(&llvm_config_path, "--includedir");
    //let llvm_lib_dir = llvm_config(&llvm_config_path, "--libdir");

    macro_rules! headers {
        ($header:tt) => { cfg.header($header); };
        ($header:tt,) => { cfg.header($header); };
        ($($header:tt),*) => {
            $(
                headers!($header);
            )*
        };
        ($($header:tt),*,) => {
            $(
                headers!($header);
            )*
        };
    }
    headers! {
        "clang-c/Index.h",
        "clang-c/Documentation.h",
        "clang-c/CXCompilationDatabase.h",
        "clang/AST/AST.h",
    }

    // Include the directory where the header files are defined
    dbg!(&llvm_include_dir);
    cfg.include(llvm_include_dir);
    cfg.language(ctest::Lang::CXX);
    cfg.flag("-std=c++11");

    cfg.skip_struct(move |s| {
        match s {
            "Clang" => true,
            _ => false,
        }
    });

    cfg.skip_signededness(|c| {
        // signededness test does not make sense for these:
        match c {
            "CXClientData" |
            "CXCursorVisitor" |
            "CXInclusionVisitor"
                => true,
            _ => false,
        }
    });

    cfg.const_cname(|c| {
        if c.starts_with("CXIndexOpt") {
            let c = c.split("CXIndexOpt").nth(1).unwrap();
            format!("CXIndexOpt_{}", c)
        } else {
            c.to_string()
        }
    });

    cfg.type_name(|ty, is_struct, _is_union| {
        match ty {
            "CXCodeCompleteResults" |
            "CXComment" |
            "CXCompletionResult" |
            "CXCursor" |
            "CXCursorAndRangeVisitor" |
            "CXFileUniqueID" |
            "CXIdxAttrInfo" |
            "CXIdxBaseClassInfo" |
            "CXIdxCXXClassDeclInfo" |
            "CXIdxContainerInfo" |
            "CXIdxDeclInfo" |
            "CXIdxEntityInfo" |
            "CXIdxEntityRefInfo" |
            "CXIdxIBOutletCollectionAttrInfo" |
            "CXIdxImportedASTFileInfo" |
            "CXIdxIncludedFileInfo" |
            "CXIdxLoc" |
            "CXIdxObjCCategoryDeclInfo" |
            "CXIdxObjCContainerDeclInfo" |
            "CXIdxObjCInterfaceDeclInfo" |
            "CXIdxObjCPropertyDeclInfo" |
            "CXIdxObjCProtocolRefInfo" |
            "CXIdxObjCProtocolRefListInfo" |
            "CXPlatformAvailability" |
            "CXSourceLocation" |
            "CXSourceRange" |
            "CXSourceRangeList" |
            "CXString" |
            "CXStringSet" |
            "CXTUResourceUsage" |
            "CXTUResourceUsageEntry" |
            "CXToken" |
            "CXType" |
            "CXUnsavedFile" |
            "CXVersion" |
            "IndexerCallbacks"
                => ty.to_string(),
            ty if is_struct => format!("struct {}", ty),
            ty => ty.to_string(),
        }
    });

    // Skip all functions. All signatures containing C enums differ because
    // clang-sys implements the C enums as integers, but integers are not
    // implicitly convertible to enums in C.
    cfg.skip_fn(|_s| true);

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}
