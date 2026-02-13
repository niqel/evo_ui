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
        } => {
            let project_dir = output.join(&name);
            create_project(&project_dir, project_type)?;
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
                anyhow::bail!("No existe el archivo UI: {}", ui_path.display());
            }

            println!("▶️  Ejecutando UI: {}", ui_path.display());

            // ⚠️ Ajusta este import/llamada según cómo quedó tu engine:
            // La idea es: runtime winit mínimo, algo como run_from_path(ui_path)
            evo_ui_engine::runtime::run_from_path(&ui_path)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            Ok(())
        }
    }
}

fn canonical_dir(path: PathBuf) -> anyhow::Result<PathBuf> {
    let p = if path.as_os_str().is_empty() { PathBuf::from(".") } else { path };
    let p = fs::canonicalize(&p)?;
    if !p.is_dir() {
        anyhow::bail!("No es un directorio: {}", p.display());
    }
    Ok(p)
}

fn create_project(project_dir: &Path, project_type: ProjectType) -> anyhow::Result<()> {
    if project_dir.exists() {
        anyhow::bail!("Ya existe: {}", project_dir.display());
    }

    fs::create_dir_all(project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("src").join("acetates"))?;

    // manifiesto mínimo (toml)
    let manifest = r#"
[app]
name = "mi_app"
type = "desktop"
"#;
    fs::write(project_dir.join("manifest.toml"), manifest.trim_start())?;

    // ui.toml inicial (simple)
    let ui_toml = r##"
[scene]
width = 800
height = 450

[[acetate]]
id = "bg"
x = 0
y = 0
w = 800
h = 450
fill = "#0b1020"

[[acetate]]
id = "panel"
z = 10
x = 80
y = 70
w = 240
h = 120
fill = "#4b14e2"
"##;
    fs::write(project_dir.join("ui.toml"), ui_toml.trim_start())?;

    // placeholder main.evo (para futuro evo_script)
    let main_evo = r#"
# main.evo (placeholder)
# aquí irá la lógica cuando exista evo_script
"#;
    fs::write(project_dir.join("src").join("main.evo"), main_evo.trim_start())?;

    // según tipo (por ahora solo marca)
    match project_type {
        ProjectType::Desktop => {}
        ProjectType::Wasm => {
            // más adelante: estructura wasm, index.html, etc.
        }
    }

    Ok(())
}
