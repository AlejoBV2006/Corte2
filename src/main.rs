use eframe::{egui::{self, ahash::{HashMapExt}}, glow::COMPLETION_STATUS};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use winapi::shared::wtypes::DATE;
use std::{collections::HashMap, fmt::format, fs::File, io::BufReader};
use chrono::prelude::Local;
use std::io::Write;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_max_inner_size([900.0, 600.0])
            .with_min_inner_size([900.0, 600.0])
            .with_resizable(false)
            .with_maximize_button(false),
            ..Default::default()
    };

    eframe::run_native("Minimercado Santi", native_options, Box::new(|_cc| {
        Box::new(ProgramApp::default())}))
}

#[derive(PartialEq, Default)]
enum Screen {
    #[default]
    Panel,
    Sell,
    Report,
    GraphicReport,
    Users
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct User {
    id: i32,
    name: String,
    pw: String
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Product {
    id: i32,
    price: i32,
    name: String,
    stock: i32
}
#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Client {
    id: i32,
    name: String,
    age: i32,
    id_shoppings: std::collections::HashMap<String, i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SaleItem {
    name: String,
    stock: i32,
    price: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Sale {
    id: u32,
    date: String, 
    id_client: i32,
    products: Vec<SaleItem>, 
    price: i32,
}

fn load_json<T>(path: String) -> Result<Vec<T>, Box<dyn std::error::Error>>
where T: for<'de> Deserialize<'de>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
 
struct ProgramApp {
    user: String,
    password: String,
    users: Vec<User>,
    clients: Vec<Client>,
    access_login: bool,
    current_screen: Screen,
    product: String,
    products: Vec<Product>,
    error_stock: String,
    cart: Vec<SaleItem>,
    price_all: i32,
    edit_user: bool,
    new_user: bool,
    delete_user: bool,
    client_selected: String,
    edit_client: bool,
    new_client: bool,
    client_name: String,
    client_age: String,
    delete_client: bool,
    sales_history: Vec<Sale>
}   

impl ProgramApp {
    fn generator_report(&self) {
        let mut file = File::create("Informe.txt").expect("Error");
        let date = format!("       Fecha: {}", Local::now().to_string());
        writeln!(file, "==========================================").unwrap();
        writeln!(file, "       INFORME DIARIO DE VENTAS").unwrap();
        writeln!(file, "{}" ,date).unwrap();
        writeln!(file, "==========================================\n").unwrap();

        let mut total = 0;

        for sale in &self.sales_history {
            writeln!(file, "Venta ID: {} | Cliente ID: {}", sale.id, sale.id_client).unwrap();
            writeln!(file, "Hora: {}", sale.date).unwrap();
            writeln!(file, "------------------------------------------").unwrap();
            for item in &sale.products {
                writeln!(
                    file, 
                    " - {:<15} x{:>2} | Subtotal: ${:>7}", 
                    item.name, item.stock, item.price
                ).unwrap();
            }

            writeln!(file, "TOTAL VENTA: ${}\n", sale.price).unwrap();
            total += sale.price;
        }

        writeln!(file, "==========================================").unwrap();
        writeln!(file, "TOTAL RECAUDADO EN EL DÍA: ${}", total).unwrap();
        writeln!(file, "==========================================").unwrap();
        
        println!("Informe generado con éxito en 'informe_diario.txt'");
    }
}

impl Default for ProgramApp {
    fn default() -> Self {
        let users_default = load_json("users.json".to_string()).unwrap_or_else(|_| Vec::new());
        let clients_default = load_json("clientes.json".to_string()).unwrap_or_else(|_| Vec::new());
        let products_default = load_json("productos.json".to_string()).unwrap_or_else(|_| Vec::new());   
        Self {  
            user: String::new(),
            password: String::new(),
            clients: clients_default,
            users: users_default,
            access_login: false,
            current_screen: Screen::default(),
            product: String::new(),
            products: products_default,
            error_stock: String::new(),
            cart: Vec::new(),
            price_all: 0,
            edit_user: false,
            new_user: false,
            edit_client: false,
            new_client: false,
            client_selected: String::new(),
            client_name: String::new(),
            client_age: String::new(),
            delete_client: false,
            delete_user: false,
            sales_history: Vec::new()
        }
    }
}

impl eframe::App for ProgramApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.edit_client {
            egui::Window::new("Editar cliente")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(100.0, 100.0))
            .show(ctx, |ui| {
                if let Some(client) = self.clients.iter_mut().find(|c| c.id.to_string() == self.client_selected) {
                    ui.label(format!("Editar usuario {}", client.name));
                    ui.vertical(|ui|{
                        ui.allocate_ui(egui::vec2(50.0, 25.0), |ui|{
                            egui::Grid::new("grid_edit_client")
                                .num_columns(2)
                                .spacing([15.0, 20.0])
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.add(egui::TextEdit::singleline(&mut self.client_name)
                                        .desired_width(200.0));
                                    ui.end_row();
                                    ui.add(egui::TextEdit::singleline(&mut self.client_age)
                                        .desired_width(200.0));
                                    ui.end_row();
                                    if ui.add(egui::Button::new("Confirmar").min_size(egui::vec2(50.0, 25.0))).clicked() {
                                        client.name = self.client_name.to_string();
                                        client.age = self.client_age.parse().expect("Error");
                                        self.edit_client = false;
                                        self.client_name.clear();
                                        self.client_age.clear();
                                    }
                                });
                        });
                    });
                };
            });
        }

        if self.delete_client {
            self.delete_client = false;
            self.clients.retain(|c| c.id.to_string() != self.client_selected);
        }

        if self.new_client {
            egui::Window::new(" Crear cliente")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(100.0, 100.0))
            .show(ctx, |ui| {
                ui.label("Crear Usuario");
                ui.vertical(|ui|{
                    ui.allocate_ui(egui::vec2(50.0, 25.0), |ui|{
                        egui::Grid::new("grid_new_client")
                            .num_columns(2)
                            .spacing([15.0, 20.0])
                            .min_col_width(100.0)
                            .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.client_name)
                                    .desired_width(200.0));
                                ui.end_row();
                                ui.add(egui::TextEdit::singleline(&mut self.client_age)
                                    .desired_width(200.0));
                                ui.end_row();
                                if ui.add(egui::Button::new("Confirmar").min_size(egui::vec2(50.0, 25.0))).clicked() {
                                    if let Some(last_client) = self.clients.last() {
                                        self.clients.push(Client {
                                            id: last_client.id + 1,
                                            name: self.client_name.clone(),
                                            age: self.client_age.parse().expect("Error"),
                                            id_shoppings: std::collections::HashMap::new()
                                        });
                                    } else {
                                        self.clients.push(Client { id: 0, name: self.client_name.clone(), age: self.client_age.parse().expect("Error"), id_shoppings: HashMap::new() });
                                    }
                                    self.new_client = false;
                                    self.client_name.clear();
                                    self.client_age.clear();
                                }
                            });
                    });
                });
            }); 
        }

        if self.edit_user {
            egui::Window::new("Editar usuario")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(100.0, 100.0))
            .show(ctx, |ui| {
                if let Some(user) = self.users.iter_mut().find(|c| c.id.to_string() == self.client_selected) {
                    ui.label(format!("Editar usuario {}", user.name));
                    ui.vertical(|ui|{
                        ui.allocate_ui(egui::vec2(50.0, 25.0), |ui|{
                            egui::Grid::new("grid_edit_user")
                                .num_columns(2)
                                .spacing([15.0, 20.0])
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.add(egui::TextEdit::singleline(&mut self.client_name)
                                        .desired_width(200.0));
                                    ui.end_row();
                                    ui.add(egui::TextEdit::singleline(&mut self.client_age)
                                        .desired_width(200.0));
                                    ui.end_row();
                                    if ui.add(egui::Button::new("Confirmar").min_size(egui::vec2(50.0, 25.0))).clicked() {
                                        user.name = self.client_name.to_string();
                                        user.pw = self.client_age.to_string();
                                        self.edit_user = false;
                                        self.client_name.clear();
                                        self.client_age.clear();
                                    }
                                });
                        });
                    });
                };
            });
        }

        if self.delete_user {
            self.delete_user = false;
            self.users.retain(|c| c.id.to_string() != self.client_selected);
        }
        
        if self.new_user {
            egui::Window::new("Crear usuario")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(100.0, 100.0))
            .show(ctx, |ui| {
                ui.label("Crear Usuario");
                ui.vertical(|ui|{
                    ui.allocate_ui(egui::vec2(50.0, 25.0), |ui|{
                        egui::Grid::new("grid_new_user")
                            .num_columns(2)
                            .spacing([15.0, 20.0])
                            .min_col_width(100.0)
                            .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.client_name)
                                    .desired_width(200.0));
                                ui.end_row();
                                ui.add(egui::TextEdit::singleline(&mut self.client_age)
                                    .desired_width(200.0));
                                ui.end_row();
                                if ui.add(egui::Button::new("Confirmar").min_size(egui::vec2(50.0, 25.0))).clicked() {
                                    if let Some(last_user) = self.users.last() {
                                        self.users.push(User { 
                                            id: last_user.id + 1, 
                                            name: self.client_name.clone(), 
                                            pw: self.client_age.clone()
                                        });
                                    } else {
                                        self.users.push(User { id: 1, name: self.client_name.clone(), pw: self.client_age.clone()});
                                    }
                                    self.new_user = false;
                                    self.client_name.clear();
                                    self.client_age.clear();
                                }
                            });
                    });
                });
            }); 
        }

        if self.access_login {
            egui::SidePanel::left("side_panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("Acciones");
                    ui.separator();
    
                if ui.selectable_label(self.current_screen == Screen::Panel, "Panel").clicked() {
                    self.current_screen = Screen::Panel;
                }
                if ui.selectable_label(self.current_screen == Screen::Sell, "Vender").clicked() {
                    self.current_screen = Screen::Sell;
                }
                if ui.selectable_label(self.current_screen == Screen::Report, "Informe Diario").clicked() {
                    self.current_screen = Screen::Report;
                }
                if ui.selectable_label(self.current_screen == Screen::GraphicReport, "Informe Gráfico").clicked() {
                    self.current_screen = Screen::GraphicReport;
                }

                if self.user == "admin" {
                    if ui.selectable_label(self.current_screen == Screen::Users, "Usuarios").clicked() {
                        self.current_screen = Screen::Users;
                    }
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.access_login{
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);

                        ui.allocate_ui(egui::vec2(400.0, 200.0), |ui| {
                            egui::Grid::new("grid_init_session")
                                .num_columns(2)
                                .spacing([15.0, 20.0])
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new("Username ").strong());
                                        });

                                    ui.horizontal(|ui| {
                                        ui.add(egui::TextEdit::singleline(&mut self.user)
                                            .hint_text("username")
                                            .desired_width(200.0))
                                        });
                                    ui.end_row();
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new("Password ").strong());
                                        });

                                    ui.horizontal(|ui| {
                                        ui.add(egui::TextEdit::singleline(&mut self.password)
                                            .password(true)
                                            .hint_text("Password")
                                            .desired_width(200.0))
                                            });
                                    ui.end_row();

                                    ui.label("");
                                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) || ui.add(egui::Button::new("Confirm").min_size(egui::vec2(200.0, 30.0))).clicked() {
                                        if !self.user.is_empty() && !self.password.is_empty(){
                                            for user in &self.users {
                                                if user.name.to_lowercase() == self.user.to_lowercase() && user.pw == self.password{
                                                    self.access_login = true;
                                                }
                                            }
                                        }
                                    }
                                    ui.end_row();
                    }); 
                });
            });
            }

            if self.access_login == true{
                match self.current_screen{
                    Screen::Panel => {
                        ui.vertical_centered(|ui| {
                            let table = TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::remainder());

                            table.header(20.0, |mut header| {
                                header.col(|ui| {ui.strong("ID");});
                                header.col(|ui| {ui.strong("Nombre");});
                                header.col(|ui| {ui.strong("Edad");});
                                header.col(|ui| {ui.strong("Cantidad de compras");});
                                header.col(|ui| {ui.strong("Panel admin");});
                                //header.col(|ui| {ui.strong("ID")});
                            })
                            .body(|mut body| {
                                for client in &self.clients {
                                    let mut quantity_shop = 0;
                                    body.row(25.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(client.id.to_string());
                                        });
                                        row.col(|ui| {
                                            ui.label(client.name.to_string());
                                        });
                                        row.col(|ui| {
                                            ui.label(client.age.to_string());
                                        });
                                        row.col(|ui| {
                                            for product in &self.products {
                                                if let Some(quantity) = client.id_shoppings.get(&product.name) {
                                                    quantity_shop += quantity;
                                                }
                                            }
                                            ui.label(quantity_shop.to_string());
                                        });
                                        row.col(|ui| {
                                            if self.user == "admin"{
                                                if ui.add(egui::Button::new("Editar").min_size(egui::vec2(100.0, 20.0))).clicked() {
                                                    self.edit_client = true;
                                                    self.client_selected = client.id.to_string();
                                                    self.client_name = client.name.to_string();
                                                    self.client_age = client.age.to_string();
                                                }
                                                if ui.add(egui::Button::new("Eliminar").min_size(egui::vec2(100.0, 20.0))).clicked() {
                                                    self.client_selected = client.id.to_string();
                                                    self.delete_client = true;
                                                }
                                            } else {
                                                ui.label("Solo admin puede ejecutar esto :C");
                                            }
                                        });
                                    });
                                }
                        });
                        if ui.add(egui::Button::new("Nuevo Cliente").min_size(egui::vec2(100.0, 50.0))).clicked() {
                            self.new_client = true;
                        }

                        });
                    }

                    Screen::Sell => {
                        ui.columns(2, |columns| {
                                columns[0].vertical(|ui| {
                                    ui.heading("Nueva Venta");
                                    ui.add_space(10.0);

                                    egui::Grid::new("grid_venta")
                                        .num_columns(2)
                                        .spacing([15.0, 20.0])
                                        .show(ui, |ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(egui::RichText::new("Cliente ").strong());
                                            });

                                            ui.horizontal(|ui| {
                                                ui.add(egui::TextEdit::singleline(&mut self.client_selected)
                                                    .hint_text("Id cliente")
                                                    .desired_width(150.0));
                                            });

                                            ui.end_row();
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(egui::RichText::new("Producto ").strong());
                                            });

                                            ui.horizontal(|ui| {
                                                ui.add(egui::TextEdit::singleline(&mut self.product)
                                                    .hint_text("Nombre del producto")
                                                    .desired_width(150.0));
                                            });
                                            ui.end_row();
                                            let input = self.product.trim().to_string();
                                            let input_client = self.client_selected.trim().to_string();
                                            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !input.is_empty() && !input_client.is_empty(){
                                                ui.add_space(20.0);         
                                                ui.separator();
                                                if let Some(_client) = self.clients.iter_mut().find(|c| c.id.to_string() == input_client) {
                                                    let (quantity, name) = if let Some((q, n)) = input.split_once("x") {
                                                        let stock_shop = q.parse::<i32>().unwrap_or(1);
                                                        (stock_shop, n.trim().to_string())
                                                    } else {
                                                        (1, input.clone())
                                                    };
    
                                                    for product in self.products.iter_mut() {
                                                        if name == product.name.to_string() {
                                                            if product.stock <= 0 || quantity > product.stock {
                                                                self.error_stock = "Lo siento no hay de ese producto".to_string();
                                                            } else {
                                                                product.stock -= quantity;
                                                                self.cart.push(SaleItem { name: product.name.clone(), stock: quantity, price: product.price * quantity });
                                                                self.price_all += product.price * quantity;
                                                                self.error_stock.clear();
                                                            }
                                                        }                        
                                                        self.product.clear();
                                                    }
                                                }
                                            }

                                        });
                                        
                                        if !self.error_stock.is_empty() {
                                            ui.label(self.error_stock.to_string());
                                        }

                                        egui::ScrollArea::vertical()
                                            .id_source("carrito_scroll")
                                            .max_height(300.0)
                                            .show(ui, |ui| {
                                                for product_table in &self.cart {
                                                    ui.horizontal(|ui| {
                                                        ui.label(egui::RichText::new(format!("{}x", product_table.stock)).strong().color(egui::Color32::GOLD));
                                                        ui.label(&product_table.name);
                                                        
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                            ui.label(format!("${}", product_table.price));
                                                        });
                                                    });
                                                }
                                            });
                                        
                                        ui.label(egui::RichText::new(format!("Precio total: {}", self.price_all)).strong().color(egui::Color32::GOLD));
                                        ui.horizontal(|ui|{
                                            if ui.add(egui::Button::new("Confirmar")).clicked() {
                                            if !self.cart.is_empty(){
                                                if let Some(client) = self.clients.iter_mut().find(|c| c.id.to_string() == self.client_selected) {
                                                    for i in self.cart.iter_mut() {
                                                        *client.id_shoppings.entry(i.name.clone()).or_insert(0) += i.stock;  
                                                    }

                                                    let date_local = Local::now();
                                                    self.sales_history.push(Sale {
                                                        id: (self.sales_history.len() + 1) as u32,
                                                        date: date_local.to_string(),
                                                        id_client: client.id,
                                                        products: self.cart.clone(),
                                                        price: self.price_all.clone()
                                                    });
                                                }
                                                self.price_all = 0;
                                                self.cart.clear();
                                                println!("{:#?}", self.sales_history);
                                            }
                                            };

                                            if ui.add(egui::Button::new("Eliminar")).clicked() {                                                
                                                if !self.cart.is_empty(){
                                                    let mut stock: i32 = 0;
                                                    for i in &self.cart {
                                                        stock += i.stock;
                                                    }
                                                    for i in self.products.iter_mut() {
                                                        i.stock += stock;
                                                        self.price_all = 0;
                                                    }
                                                    self.cart.clear();
                                                };
                                            }    

                                        });
                                });

                                columns[1].vertical(|ui| {
                                    ui.heading("Stock Disponible");
                                    ui.separator();

                                    egui::ScrollArea::vertical().id_source("inventory_scroll").show(ui, |ui| {

                                        for product in &self.products {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(format!("{}x", product.stock)).strong());
                                                ui.label(product.name.to_string());
                                            });
                                        }
                                    });
                                });
                            });
                    }

                    Screen::Report => {
                        ui.heading("Historial de Ventas del Día");

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            let mut total = 0;
                            
                            for sale in &self.sales_history {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("ID: #{}", sale.id));
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(egui::RichText::new(format!("${}", sale.price)).strong().color(egui::Color32::GREEN));
                                        });
                                    });
                                    ui.label(format!("Cliente: {}", sale.id_client));
                                    ui.small(&sale.date);
                                });
                                total += sale.price;
                            }
                            
                            ui.separator();
                            ui.heading(format!("Total acumulado: ${}", total));
                            
                            if ui.button("💾 Exportar Informe a TXT").clicked() {
                                self.generator_report();
                            }
                        });

                    }

                    Screen::GraphicReport => {
                        let mut total_product: HashMap<String, i32> = HashMap::new();
                        let mut max_venta = 0;

                        for sale in &self.sales_history {
                            for item in &sale.products {
                                let total = total_product.entry(item.name.clone()).or_insert(0);
                                *total += item.stock;
                                if *total > max_venta { max_venta = *total; }
                            }
                        }

                        // --- SOLUCIÓN AL ORDEN: Convertimos a Vec y ordenamos por cantidad (descendente) ---
                        let mut sorted_products: Vec<_> = total_product.iter().collect();
                        sorted_products.sort_by(|a, b| b.1.cmp(a.1));

                        ui.columns(3, |columns| {
                            let total_money: i32 = self.sales_history.iter().map(|s| s.price).sum();
                            let total_items: i32 = self.sales_history.iter().map(|s| s.products.len() as i32).sum();

                            columns[0].vertical_centered(|ui| {
                                ui.label("💰 Total en Caja");
                                ui.heading(format!("${}", total_money));
                            });
                            columns[1].vertical_centered(|ui| {
                                ui.label("📦 Items Vendidos");
                                ui.heading(format!("{}", total_items));
                            });
                            columns[2].vertical_centered(|ui| {
                                ui.label("🧾 Facturas");
                                ui.heading(format!("{}", self.sales_history.len()));
                            });
                        });
                        
                        ui.separator();                        
                        ui.vertical_centered(|ui| {
                            ui.heading(egui::RichText::new("📊 Reporte de Movimiento de Inventario").strong().size(25.0));
                        });
                        ui.add_space(20.0);

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (nombre, cantidad) in sorted_products {
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new(egui::RichText::new(format!("{:<15}", nombre)).monospace()).truncate(true));
                                    
                                    let max_width = ui.available_width() - 100.0;
                                    let percentage = if max_venta > 0 { *cantidad as f32 / max_venta as f32 } else { 0.0 };
                                    let width_bar = (max_width * percentage).max(2.0); 
                                    ui.push_id(nombre, |ui| {
                                        let (rect, _response) = ui.allocate_at_least(egui::vec2(width_bar, 20.0), egui::Sense::hover());
                                        
                                        let color = if _response.hovered() {
                                            egui::Color32::from_rgb(120, 170, 255)
                                        } else {
                                            egui::Color32::from_rgb(100, 150, 250)
                                        };

                                        ui.painter().rect_filled(rect, 4.0, color);
                                    });

                                    ui.label(format!(" {} uds", cantidad));
                                });
                                ui.add_space(8.0);
                            }
                        });

                        if total_product.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("No hay ventas registradas todavía").italics());
                            });
                        }                    
                    }

                    Screen::Users => {
                        ui.vertical_centered(|ui| {
                            let table = TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::remainder());
                            table.header(20.0, |mut header| {
                                header.col(|ui| {ui.strong("ID");});
                                header.col(|ui| {ui.strong("Usuario");});
                                header.col(|ui| {ui.strong("Contraseña");});
                                header.col(|ui| {ui.strong("Acciones");});
                            })
                            .body(|mut body| {
                                for user in &self.users {
                                    if !(user.name == "admin") {
                                        body.row(25.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(user.id.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(user.name.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(user.pw.to_string());
                                            });
                                            row.col(|ui| {
                                                if ui.add(egui::Button::new("Editar").min_size(egui::vec2(100.0, 20.0))).clicked() {
                                                    self.edit_user = true;
                                                    self.client_selected = user.id.to_string();
                                                    self.client_age = user.pw.clone();
                                                    self.client_name = user.name.clone();
                                                }
                                                if ui.add(egui::Button::new("Eliminar").min_size(egui::vec2(100.0, 20.0))).clicked() {
                                                    self.delete_user = true;
                                                    self.client_selected = user.id.to_string();
                                                }
                                            });
                                        });
                                        
                                    }
                                }
                            });

                            if ui.add(egui::Button::new("Nuevo usuario").min_size(egui::vec2(100.0, 30.0))).clicked() {
                                self.new_user = true;

                            }
                        }); 
                    }
                }
            }
        });
    }
}
