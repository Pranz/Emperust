
/home/pranz/Elements/Programming/Rust/emperust2/src/biome.rs,422
pub enum Biome {Biome5,81
pub enum BiomeType {BiomeType36,798
pub enum BiomeRepresentation {BiomeRepresentation47,922
impl Biome {Biome54,1026
    pub fn new(height: u8, temperature: u8, rainfall: u8, tree_line: u8, ocean_line: u8) -> Biome {new55,1039
    pub fn graphical_representation(self) -> BiomeRepresentation {graphical_representation80,2534
    pub fn category(self) -> BiomeType {category135,5203

/home/pranz/Elements/Programming/Rust/emperust2/src/direction.rs,129
pub enum Direction {Direction5,58
impl Direction {Direction12,121
    pub fn to_point(&self) -> Point<i32> {to_point13,138

/home/pranz/Elements/Programming/Rust/emperust2/src/game.rs,485
pub struct Game {Game15,325
pub enum ProgressInfo {ProgressInfo27,629
impl Game {Game32,692
    pub fn new(settings: Settings, tx: Sender<ProgressInfo>) -> Game {new33,704
    pub fn execute_command(&mut self, cmd: UserCommand) {execute_command64,2332
    pub fn regenerate_map(&mut self) {regenerate_map82,2906
    pub fn move_cursor(&mut self, dpos: Point<i32>) {move_cursor99,3753
    pub fn get_zoomed_out_cursor(&self) -> Point<i32> {get_zoomed_out_cursor137,5486

/home/pranz/Elements/Programming/Rust/emperust2/src/input.rs,115
pub enum UserCommand {UserCommand8,125
pub fn handle_input(root: &mut Root) -> UserCommand {handle_input15,219

/home/pranz/Elements/Programming/Rust/emperust2/src/main.rs,258
mod settings;settings15,269
mod input;input16,283
mod direction;direction17,294
mod point;point18,309
mod game;game19,320
mod render;render20,330
mod map;map21,342
mod biome;biome22,351
mod world_gen;world_gen23,362
fn main() {main30,505

/home/pranz/Elements/Programming/Rust/emperust2/src/map.rs,1720
pub struct Tile {Tile18,438
impl Tile {Tile26,580
    pub fn graphical_representation(self, map: &Map) -> (char, Color, Color) {graphical_representation27,592
pub struct Map {Map74,2754
pub struct ZoomedMap {ZoomedMap83,2974
pub type DiscreteField = Box<Fn(usize, usize) -> u8>;DiscreteField89,3076
impl Map {Map91,3131
    pub fn new(width: usize,new92,3142
    pub fn create_rivers(&mut self, amount: usize) -> Vec<Vec<(usize, usize)>> {create_rivers130,4388
    pub fn create_river(&mut self, x_orig: usize, y_orig: usize) -> Vec<(usize, usize)> {create_river150,5141
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {in_bounds195,7006
    pub fn neighbour_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {neighbour_positions199,7113
    pub fn in_bounds_isize(&self, x: isize, y: isize) -> bool {in_bounds_isize214,7618
    pub fn get_tile(&self, x: usize, y: usize) -> Tile {get_tile219,7781
    pub fn get_height(&self, x: usize, y: usize) -> u8 {get_height230,8188
    pub fn get_biome(&self, x: usize, y: usize) -> Biome {get_biome235,8319
    pub fn set_biome(&mut self, x: usize, y: usize, biome: Biome) {set_biome239,8429
impl ZoomedMap {ZoomedMap244,8559
    pub fn get_biome(&self, x: usize, y: usize) -> Biome {get_biome245,8576
pub fn get_height_map(settings: &Settings) -> Box<Fn(usize, usize) -> u8> {get_height_map253,8735
pub fn get_temperature_map(settings: &Settings) -> DiscreteField {get_temperature_map273,9715
pub fn get_rainfall_map(settings: &Settings) -> Box<Fn(usize, usize, u8) -> u8> {get_rainfall_map292,10760
pub fn zoomed_map(map: &Map, width: usize, height: usize, settings: &Settings) -> ZoomedMap {zoomed_map312,11770

/home/pranz/Elements/Programming/Rust/emperust2/src/point.rs,731
pub struct Point<T> {Point5,78
impl<T> Point<T> {Point10,131
    pub fn new(x: T, y: T) -> Point<T>{new11,150
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Point<T> {map16,250
impl<T: Add> Add for Point<T> {Add for Point21,358
    type Output = Point<<T as Add>::Output>;Output22,390
    fn add(self, other: Point<T>) -> Point<<T as Add>::Output> {add25,458
impl<T: Neg> Neg for Point<T> {Neg for Point30,589
    type Output = Point<<T as Neg>::Output>;Output31,621
    fn neg(self) -> Point<<T as Neg>::Output> {neg34,689
impl<T : Sub> Sub for Point<T> {Sub for Point39,796
    type Output = Point<<T as Sub>::Output>;Output40,829
    fn sub(self, other: Point<T>) -> Point<<T as Sub>::Output> {sub43,897

/home/pranz/Elements/Programming/Rust/emperust2/src/render.rs,408
pub fn render_screen(game: &mut Game, root: &mut Root) {render_screen12,223
pub fn render_map_zoomed_in(game: &mut Game) {render_map_zoomed_in26,731
pub fn render_map_zoomed_out(game: &mut Game) {render_map_zoomed_out45,1385
pub fn render_debug_info(game: &mut Game) {render_debug_info62,2212
pub fn render_progress(root: &mut Root, width: usize, rx: Receiver<ProgressInfo>){render_progress80,3012

/home/pranz/Elements/Programming/Rust/emperust2/src/settings.rs,143
pub struct Settings {Settings8,87
impl Settings {Settings37,984
    pub fn read(path: &'static str) -> io::Result<Settings> {read38,1000

/home/pranz/Elements/Programming/Rust/emperust2/src/world_gen.rs,488
pub type ScalarField = Box<Fn(f32, f32) -> f32>;ScalarField16,238
pub fn get_noise_map(lacunarity: f32, hurst: f32, coefficient: f32) -> ScalarField {get_noise_map18,288
pub fn combine_scalar_fields(scalar_fields: Vec<(ScalarField, f32)>) -> ScalarField {combine_scalar_fields29,646
pub fn get_distance_map(map_width: f32, map_height: f32) -> ScalarField {get_distance_map39,911
pub fn get_distance_vertical_map(map_height: f32) -> ScalarField {get_distance_vertical_map49,1249
/home/pranz/.rusty-tags/cache/itertools-0.4.3.emacs,include
/home/pranz/.rusty-tags/cache/num-0.1.28.emacs,include
/home/pranz/.rusty-tags/cache/rand-0.3.12.emacs,include
/home/pranz/.rusty-tags/cache/tcod-4114d2c446cdb92142b9d0c3e548126203276c24.emacs,include
/home/pranz/.rusty-tags/cache/yaml-rust-0.2.2.emacs,include
