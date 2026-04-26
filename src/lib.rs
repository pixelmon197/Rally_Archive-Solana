use anchor_lang::prelude::*;

declare_id!("AXpmHPjvcE9JvNqHjTW9sS1UY4MmPqLquxxo11RC7Gwo");

#[program]
pub mod rally_archive {
    use super::*;

    //////////////////////////// Crear Archivo /////////////////////////////////////
    pub fn crear_archivo(context: Context<NuevoArchivo>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let autos: Vec<RallyCar> = Vec::new();

        context.accounts.archivo.set_inner(RallyArchive {
            owner: owner_id,
            nombre_archivo: nombre,
            autos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Auto /////////////////////////////////////
    #[allow(clippy::too_many_arguments)]
    pub fn agregar_auto(
        context: Context<NuevoAuto>,
        nombre: String,
        marca: String,
        anio: u16,
        motor: String,
        velocidad_max: u16,
        grupo: String,
        traccion: String,
        caballos: u16,
        peso: u16,
        piloto_famoso: String,
        victorias: u8,
        evento_destacado: String,
        pais_evento: String,
        calificacion: u8,
    ) -> Result<()> {
        require!(
            context.accounts.archivo.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let auto = RallyCar {
            nombre,
            marca,
            anio,
            motor,
            velocidad_max,
            grupo,
            traccion,
            caballos,
            peso,
            piloto_famoso,
            victorias,
            evento_destacado,
            pais_evento,
            calificacion,
            activo: true,
        };

        context.accounts.archivo.autos.push(auto);

        Ok(())
    }

    //////////////////////////// Eliminar Auto /////////////////////////////////////
    pub fn eliminar_auto(context: Context<NuevoAuto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.archivo.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let autos = &mut context.accounts.archivo.autos;

        for i in 0..autos.len() {
            if autos[i].nombre == nombre {
                autos.remove(i);
                msg!("Auto {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::AutoNoExiste.into())
    }

    //////////////////////////// Ver Autos /////////////////////////////////////
    pub fn ver_autos(context: Context<NuevoAuto>) -> Result<()> {
        require!(
            context.accounts.archivo.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de autos: {:#?}", context.accounts.archivo.autos);
        Ok(())
    }

    //////////////////////////// Alternar Estado /////////////////////////////////////
    pub fn alternar_estado(context: Context<NuevoAuto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.archivo.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let autos = &mut context.accounts.archivo.autos;

        for i in 0..autos.len() {
            let estado = autos[i].activo;

            if autos[i].nombre == nombre {
                let nuevo_estado = !estado;
                autos[i].activo = nuevo_estado;

                msg!(
                    "El auto: {} ahora tiene estado activo: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::AutoNoExiste.into())
    }
}

//////////////////////////// Errores /////////////////////////////////////
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario del archivo")]
    NoEresElOwner,
    #[msg("Error, el auto no existe")]
    AutoNoExiste,
}

//////////////////////////// Cuenta Principal /////////////////////////////////////
#[account]
#[derive(InitSpace)]
pub struct RallyArchive {
    owner: Pubkey,

    #[max_len(60)]
    nombre_archivo: String,

    #[max_len(10)]
    autos: Vec<RallyCar>,
}

//////////////////////////// Struct Auto /////////////////////////////////////
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct RallyCar {
    #[max_len(60)]
    nombre: String,

    #[max_len(30)]
    marca: String,

    anio: u16,

    #[max_len(30)]
    motor: String,

    velocidad_max: u16,

    #[max_len(20)]
    grupo: String,

    #[max_len(10)]
    traccion: String,

    caballos: u16,
    peso: u16,

    #[max_len(40)]
    piloto_famoso: String,

    victorias: u8,

    #[max_len(40)]
    evento_destacado: String,

    #[max_len(30)]
    pais_evento: String,

    calificacion: u8,

    activo: bool,
}

//////////////////////////// Contextos /////////////////////////////////////
#[derive(Accounts)]
pub struct NuevoArchivo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = RallyArchive::INIT_SPACE + 8,
        seeds = [b"archivo", owner.key().as_ref()],
        bump
    )]
    pub archivo: Account<'info, RallyArchive>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoAuto<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub archivo: Account<'info, RallyArchive>,
}
