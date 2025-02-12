fn main() {
    use tera::{Context, Tera};

    println!("cargo:rerun-if-env-changed=PHALA_GIT_REVISIOIN");

    let tera = Tera::new("proto/*.proto").unwrap();

    let tmpdir = tempdir::TempDir::new("rendered_proto").unwrap();
    let render_dir = tmpdir.path();

    for tmpl in tera.templates.keys() {
        println!("cargo:rerun-if-changed=proto/{}", tmpl);
        let render_output = std::fs::File::create(render_dir.join(tmpl)).unwrap();
        tera.render_to(tmpl, &Context::new(), render_output)
            .unwrap();
    }

    let out_dir = "./src/proto_generated";

    let mut builder = prpc_build::configure()
        .out_dir(out_dir)
        .mod_prefix("crate::prpc::")
        .disable_package_emission();
    for r#type in ["InitRuntimeResponse", "Attestation", "AttestationReport"] {
        builder = builder.type_attribute(
            r#type,
            "#[cfg_attr(feature = \"serde\", derive(::serde::Serialize, ::serde::Deserialize))]",
        )
    }
    builder
        .compile(&["pruntime_rpc.proto"], &[render_dir])
        .unwrap();
}
