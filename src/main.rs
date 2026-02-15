use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

/// evo_ui — CLI para crear y ejecutar proyectos basados en evo_ui_engine
#[derive(Parser, Debug)]
#[command(name = "evo_ui", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Crea un nuevo proyecto (estructura mínima)
    Create {
        /// Nombre del proyecto (carpeta)
        #[arg(short = 'n', long = "name")]
        name: String,

        /// Directorio de salida
        #[arg(short = 'o', long = "output", default_value = ".")]
        output: PathBuf,

        /// Tipo de proyecto
        #[arg(long = "type", value_enum, default_value_t = ProjectType::Desktop)]
        project_type: ProjectType,

        /// Crea proyecto vacío (sin ui.toml)
        #[arg(long = "empty", conflicts_with = "bg_green")]
        empty: bool,

        /// Crea ui.toml mínimo con fondo verde
        #[arg(long = "bg-green", conflicts_with = "empty")]
        bg_green: bool,
    },

    /// Ejecuta el proyecto (lee ui.toml y abre ventana)
    Run {
        /// Path del proyecto (carpeta)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Archivo UI (si no se da, usa ui.toml en la raíz)
        #[arg(long = "ui", default_value = "ui.toml")]
        ui: PathBuf,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum ProjectType {
    Desktop,
    Wasm,
    // Android, Mac, Linux, Windows… (podemos agregar luego)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            name,
            output,
            project_type,
            empty,
            bg_green,
        } => {
            let project_dir = output.join(&name);
            let ui_mode = if empty {
                UiMode::Empty
            } else if bg_green {
                UiMode::BgGreen
            } else {
                UiMode::Default
            };
            create_project(&project_dir, &name, project_type, ui_mode)?;
            println!("✅ Proyecto creado en: {}", project_dir.display());
            Ok(())
        }

        Commands::Run { path, ui } => {
            let project_dir = canonical_dir(path)?;
            let ui_path = if ui.is_absolute() {
                ui
            } else {
                project_dir.join(ui)
            };

            if !ui_path.exists() {
                println!(
                    "⚠️  No existe el archivo UI: {}. Ejecutando fallback del engine.",
                    ui_path.display()
                );
            }

            println!("▶️  Ejecutando UI: {}", ui_path.display());

            // ⚠️ Ajusta este import/llamada según cómo quedó tu engine:
            // La idea es: runtime winit mínimo, algo como run_from_path(ui_path)
            evo_ui_engine::runtime::run_from_path(&ui_path).map_err(|e| anyhow::anyhow!("{e}"))?;
            Ok(())
        }
    }
}

fn canonical_dir(path: PathBuf) -> anyhow::Result<PathBuf> {
    let p = if path.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        path
    };
    let p = fs::canonicalize(&p)?;
    if !p.is_dir() {
        anyhow::bail!("No es un directorio: {}", p.display());
    }
    Ok(p)
}

#[derive(Debug, Clone, Copy)]
enum UiMode {
    Empty,
    BgGreen,
    Default,
}

fn create_project(
    project_dir: &Path,
    project_name: &str,
    project_type: ProjectType,
    ui_mode: UiMode,
) -> anyhow::Result<()> {
    if project_dir.exists() {
        anyhow::bail!("Ya existe: {}", project_dir.display());
    }

    fs::create_dir_all(project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("src").join("acetates"))?;

    // manifiesto mínimo (toml)
    let app_type = match project_type {
        ProjectType::Desktop => "desktop",
        ProjectType::Wasm => "wasm",
    };
    let manifest = format!(
        "[app]\nname = \"{}\"\ntype = \"{}\"\n",
        project_name, app_type
    );
    fs::write(project_dir.join("manifest.toml"), manifest)?;

    if !matches!(ui_mode, UiMode::Empty) {
        let fill = match ui_mode {
            UiMode::BgGreen => "#00aa00",
            UiMode::Default => "#070b16",
            UiMode::Empty => unreachable!(),
        };

        let ui_toml = format!(
            r#"[scene]
width = 800
height = 450

[[acetate]]
id = "bg"
x = 0
y = 0
w = 800
h = 450
fill = "{}"
"#,
            fill
        );
        fs::write(project_dir.join("ui.toml"), ui_toml)?;
    }

    // placeholder main.evo (para futuro evo_script)
    let main_evo = r#"
# main.evo (placeholder)
# aquí irá la lógica cuando exista evo_script
"#;
    fs::write(
        project_dir.join("src").join("main.evo"),
        main_evo.trim_start(),
    )?;

    // según tipo (por ahora solo marca)
    match project_type {
        ProjectType::Desktop => {}
        ProjectType::Wasm => {
            // más adelante: estructura wasm, index.html, etc.
        }
    }

    Ok(())
}
