use std::io::{Read, Result, Write};
use std::collections::HashMap;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use middle::ty::LangType;
use middle::GlobalCtx;

#[derive(Debug)]
struct FileHeader {
    magic: [u8; 4],
    version: u16,
    symbol_count: u32,
    type_count: u32,
    string_pool_size: u32,
}

impl FileHeader {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic)?;
        writer.write_u16::<LittleEndian>(self.version)?;
        writer.write_u32::<LittleEndian>(self.symbol_count)?;
        writer.write_u32::<LittleEndian>(self.type_count)?;
        writer.write_u32::<LittleEndian>(self.string_pool_size)?;
        Ok(())
    }
}

#[derive(Debug)]
struct SymbolRecord {
    name_offset: u32,
    type_index: u32,
    flags: u8,
}

impl SymbolRecord {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<LittleEndian>(self.name_offset)?;
        writer.write_u32::<LittleEndian>(self.type_index)?;
        writer.write_u8(self.flags)?;
        Ok(())
    }
}

#[derive(Debug)]
struct TypeRecord {
    data_size: u16,
    data: Vec<u8>,
}

impl TypeRecord {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16::<LittleEndian>(self.data_size)?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}

fn marshall_symbol_tables<'a, W: Write>(
    ctx: &GlobalCtx<'a>,
    writer: &mut W
) -> std::result::Result<(), std::io::Error> {
    let mut string_pool = Vec::new();
    let mut type_pool = Vec::new();
    let mut type_indices = HashMap::new();
    let mut symbol_records = Vec::new();

    for (name, hir_id) in &ctx.module_exports {
        let ty = ctx.module_ty_info.borrow()
            .get_type(hir_id).unwrap().ty.clone();

        let type_bytes = ty.to_bytes();
        let type_idx = *type_indices.entry(type_bytes.clone())
            .or_insert_with(|| {
                type_pool.push(TypeRecord {
                    data_size: type_bytes.len() as u16,
                    data: type_bytes,
                });
                (type_pool.len() - 1) as u32
            });

        let name_offset = string_pool.len() as u32;
        string_pool.extend_from_slice(name.as_bytes());
        string_pool.push(0);

        symbol_records.push(SymbolRecord {
            name_offset,
            type_index: type_idx,
            flags: 0,
        });
    }

    let header = FileHeader {
        magic: *b"COMO",
        version: 1,
        symbol_count: symbol_records.len() as u32,
        type_count: type_pool.len() as u32,
        string_pool_size: string_pool.len() as u32,
    };
    header.write(writer)?;

    writer.write_all(&string_pool)?;

    for record in symbol_records {
        record.write(writer)?;
    }

    for type_record in type_pool {
        type_record.write(writer)?;
    }

    Ok(())
}

pub fn marshall_module<'a, W: Write>(
    ctx: &GlobalCtx<'a>, 
    writer: &mut W
) -> std::result::Result<(), std::io::Error> {
    marshall_symbol_tables(ctx, writer)?;
    Ok(())
}

pub fn unmarshall_and_print<R: Read>(reader: &mut R) -> Result<()> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    
    if magic != *b"COMO" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file format"
        ));
    }

    let version = reader.read_u16::<LittleEndian>()?;
    let symbol_count = reader.read_u32::<LittleEndian>()?;
    let type_count = reader.read_u32::<LittleEndian>()?;
    let string_pool_size = reader.read_u32::<LittleEndian>()?;

    println!("=== File Header ===");
    println!("Version: {}", version);
    println!("Symbols: {}", symbol_count);
    println!("Types: {}", type_count);
    println!("String pool size: {} bytes", string_pool_size);

    let mut string_pool = vec![0u8; string_pool_size as usize];
    reader.read_exact(&mut string_pool)?;
    
    let mut symbols = Vec::with_capacity(symbol_count as usize);
    for _ in 0..symbol_count {
        let name_offset = reader.read_u32::<LittleEndian>()?;
        let type_index = reader.read_u32::<LittleEndian>()?;
        let _flags = reader.read_u8()?;
        
        symbols.push((name_offset, type_index));
    }

    let mut types = Vec::with_capacity(type_count as usize);
    for _ in 0..type_count {
        let data_size = reader.read_u16::<LittleEndian>()?;
        let mut data = vec![0u8; data_size as usize];
        reader.read_exact(&mut data)?;
        
        types.push(data);
    }

    println!("\n=== Symbols ===");
    for (name_offset, type_idx) in symbols {
        let mut name_bytes = Vec::new();
        let mut pos = name_offset as usize;
        while pos < string_pool.len() && string_pool[pos] != 0 {
            name_bytes.push(string_pool[pos]);
            pos += 1;
        }
        let name = String::from_utf8_lossy(&name_bytes);

        if let Some(type_data) = types.get(type_idx as usize) {
            let ty = LangType::from_bytes(type_data)
                .unwrap_or(LangType::UNRESOLVED);
            
            println!("{:<20} : {:?}", name, ty);
        } else {
            println!("{:<20} : [UNKNOWN TYPE]", name);
        }
    }

    Ok(())
}