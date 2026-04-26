//////////////////// Imports ////////////////////
import { PublicKey, SystemProgram } from "@solana/web3.js";

////////////////// Constantes ////////////////////
const nombre_archivo = "Grupo_B";
const owner = pg.wallet.publicKey;

console.log("Mi address:", owner.toString());
const balance = await pg.connection.getBalance(owner);
console.log(`Balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

//////////////////// PDA ////////////////////
function pdaArchivo() {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("archivo"), owner.toBuffer()],
    pg.PROGRAM_ID
  );
}

//////////////////// Crear Archivo ////////////////////
async function crearArchivo(nombre_archivo) {
  const [pda_archivo] = pdaArchivo();

  const txHash = await pg.program.methods
    .crearArchivo(nombre_archivo)
    .accounts({
      owner: owner,
      archivo: pda_archivo,
      systemProgram: SystemProgram.programId, // FIX IMPORTANTE
    })
    .rpc();

  console.log("Archivo creado:", txHash);
}

//////////////////// Agregar Autos ////////////////////
async function agregarAutos() {
  const [pda_archivo] = pdaArchivo();

  // Lancia 037
  await pg.program.methods
    .agregarAuto(
      "Lancia 037",
      "Lancia",
      1982,
      "2.0L Supercharged",
      220,
      "Grupo B",
      "RWD",
      325,
      960,
      "Walter Röhrl",
      6,
      "Rally de Montecarlo",
      "Monaco",
      10
    )
    .accounts({ owner, archivo: pda_archivo })
    .rpc();

  console.log("Lancia 037 agregado");

  // MG Metro 6R4
  await pg.program.methods
    .agregarAuto(
      "MG Metro 6R4",
      "MG",
      1985,
      "3.0L V6",
      210,
      "Grupo B",
      "AWD",
      410,
      1030,
      "Tony Pond",
      3,
      "Rally RAC",
      "Reino Unido",
      9
    )
    .accounts({ owner, archivo: pda_archivo })
    .rpc();

  console.log("MG Metro 6R4 agregado");

  // Renault 5 Turbo
  await pg.program.methods
    .agregarAuto(
      "Renault 5 Turbo",
      "Renault",
      1980,
      "1.4L Turbo",
      200,
      "Grupo B",
      "RWD",
      350,
      970,
      "Jean Ragnotti",
      4,
      "Rally de Montecarlo",
      "Francia",
      9
    )
    .accounts({ owner, archivo: pda_archivo })
    .rpc();

  console.log("Renault 5 Turbo agregado");
}

//////////////////// Ver Autos ////////////////////
async function verAutos() {
  const [pda_archivo] = pdaArchivo();

  try {
    const archivoAccount = await pg.program.account.rallyArchive.fetch(
      pda_archivo
    );

    const autos = archivoAccount.autos;

    if (!autos || autos.length === 0) {
      console.log("No hay autos registrados");
      return;
    }

    console.log("Total de autos:", autos.length);

    autos.forEach((auto, index) => {
      console.log(`
Auto #${index + 1}
Nombre: ${auto.nombre}
Marca: ${auto.marca}
Año: ${auto.anio}
Motor: ${auto.motor}
Velocidad: ${auto.velocidadMax}
Grupo: ${auto.grupo}
Tracción: ${auto.traccion}
HP: ${auto.caballos}
Peso: ${auto.peso}
Piloto: ${auto.pilotoFamoso}
Victorias: ${auto.victorias}
Evento: ${auto.eventoDestacado}
País: ${auto.paisEvento}
Calificación: ${auto.calificacion}
Activo: ${auto.activo}
`);
    });
  } catch (error) {
    console.error("Error:", error);
  }
}

//////////////////// Cambiar Estado ////////////////////
async function cambiarEstado(nombre_auto) {
  const [pda_archivo] = pdaArchivo();

  const txHash = await pg.program.methods
    .alternarEstado(nombre_auto)
    .accounts({
      owner: owner,
      archivo: pda_archivo,
    })
    .rpc();

  console.log("Estado cambiado:", txHash);
}

//////////////////// Eliminar Auto ////////////////////
async function eliminarAuto(nombre_auto) {
  const [pda_archivo] = pdaArchivo();

  const txHash = await pg.program.methods
    .eliminarAuto(nombre_auto)
    .accounts({
      owner: owner,
      archivo: pda_archivo,
    })
    .rpc();

  console.log("Auto eliminado:", txHash);
}

//////////////////// EJECUCIÓN FINAL ////////////////////

await crearArchivo(nombre_archivo);
await agregarAutos();
await verAutos();

// Cambiar estado del Lancia 037
await cambiarEstado("Lancia 037");
console.log("Estado del Lancia 037 cambiado");

await verAutos();

// Eliminar MG Metro 6R4
await eliminarAuto("MG Metro 6R4");
console.log("MG Metro 6R4 eliminado");

await verAutos();
