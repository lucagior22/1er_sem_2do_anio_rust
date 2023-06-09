// 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
// dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
// conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
// verde, azul, amarillo, blanco o negro.
// Para dichas estructuras implemente los siguientes métodos:
// ❖ ConcesionarioAuto:
// ➢ new: que pasando los parámetros correspondientes, crea un
// ConcesionarioAuto y lo retorna.
// ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
// la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
// no lo agrega y retorna false.
// ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
// ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
// ❖ Auto:
// ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
// retorna.
// ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
// ■ si es de color primario le aplica un recargo del 25%, sino le aplica un
// descuento del 10%.
// ■ si la marca es BMW le aplica un recargo del 15%-
// ■ si el año es menor a 2000 le aplica un descuento del 5%.

#[derive(Debug)]
pub enum Color{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}
impl Color{
    pub fn igual(&self, color : &Color) -> bool{
        match (self, color){
            (Color::Rojo, Color::Rojo) => true,
            (Color::Verde, Color::Verde) => true,
            (Color::Azul, Color::Azul) => true,
            (Color::Amarillo, Color::Amarillo) => true,
            (Color::Blanco, Color::Blanco) => true,
            (Color::Negro, Color::Negro) => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Auto{
    marca : String,
    modelo : String,
    año : u32,
    precio_bruto : f64,
    color : Color,
}
impl Auto{
    pub fn new(marca : String, modelo : String, año : u32, precio_bruto : f64, color : Color) -> Auto{
        Auto{
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    pub fn igual(&self, auto : &Auto) -> bool{
        self.marca == auto.marca && self.modelo == auto.modelo && self.año == auto.año && self.precio_bruto == auto.precio_bruto && self.color.igual(&auto.color)
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut rec_desc = 0.0;
        match self.color{
            Color::Rojo | Color::Amarillo | Color::Azul => rec_desc += 25.0,
            _ => rec_desc -= 10.0,
        }
        if self.marca == "BMW" {
            rec_desc += 15.0;
        }
        if self.año < 2000 {
            rec_desc -= 5.0;
        }
        self.precio_bruto + self.precio_bruto * (rec_desc / 100.0)
    }
}

#[derive(Debug)]
pub struct ConcesionarioAuto{
    nombre : String,
    direccion : String,
    cant_autos : usize,
    autos : Vec<Auto>,
}
impl ConcesionarioAuto{
    pub fn new(nombre : String, direccion : String, x : usize) -> ConcesionarioAuto{
        ConcesionarioAuto{
            nombre,
            direccion,
            cant_autos : x,
            autos : Vec::new(),
        }
    }
    pub fn agregar_auto(&mut self, auto : Auto) -> bool{
        if self.autos.len() <= self.cant_autos{
            self.autos.push(auto);
            true
        } else {
            false
        }
    }
    pub fn eliminar_auto(&mut self, auto : &Auto) -> bool{
        if !self.autos.is_empty(){
            for i in 0..self.autos.len()-1{
                if self.autos[i].igual(&auto){
                    self.autos.swap_remove(i);
                    return true
                }
            }
            false
        } else {
            false
        }
    }

    pub fn buscar_auto(&mut self, auto : Auto) -> Option<&mut Auto>{
        let mut i = 0;
        let mut index: usize = 0;
        let mut encontre = false;
        for mut i in &mut self.autos{
            if i.igual(&auto){
                return Some(i)
            }
        }
        return None
    } 
}

pub fn main(){
    let mut a1 : Auto = Auto::new("Bmw".to_string(), "m3".to_string(), 2010, 15000.0, Color::Amarillo);
    let mut a2 : Auto = Auto::new("Audi".to_string(), "R8".to_string(), 2017, 40000.0, Color::Negro);
    let mut a3 : Auto = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2005, 10000.0, Color::Rojo);
    let mut a4 : Auto = Auto::new("Fiat".to_string(), "Uno".to_string(), 1999, 8000.0, Color::Azul);
    let mut a5 : Auto = Auto::new("Dodge".to_string(), "Ram".to_string(), 2020, 17000.0, Color::Verde);
    let mut a6 : Auto = Auto::new("Bmw".to_string(), "m4".to_string(), 2023, 25000.0, Color::Negro);

    let mut conc : ConcesionarioAuto = ConcesionarioAuto::new("Concesionaria".to_string(), "Calle Falsa".to_string(), 4);

    println!("{:#?}", conc.agregar_auto(a1));
    println!("{:#?}", conc.agregar_auto(a2));
    println!("{:#?}", conc.agregar_auto(a3));
    println!("{:#?}", conc.agregar_auto(a4));
    println!("{:#?}", conc.agregar_auto(a5));

    println!("{:#?}", conc.buscar_auto(a6));

}
