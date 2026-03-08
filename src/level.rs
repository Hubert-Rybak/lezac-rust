#[derive(Clone)]
pub struct Level {
    pub width: usize, pub height: usize,
    pub bonus_target: u8, pub destruction_pct: u8,
    pub foreground: Vec<u8>, pub background: Vec<u8>,
    pub entity_data: Vec<u8>,
}
impl Level {
    pub fn tile_at(&self, x: usize, y: usize) -> u8 { if x<self.width && y<self.height { self.foreground[y*self.width+x] } else { 0 } }
    pub fn set_tile(&mut self, x: usize, y: usize, v: u8) { if x<self.width && y<self.height { self.foreground[y*self.width+x] = v; } }
    pub fn bg_tile_at(&self, x: usize, y: usize) -> u8 { if x<self.width && y<self.height { self.background[y*self.width+x] } else { 0 } }
    pub fn count_solid_tiles(&self) -> usize { self.foreground.iter().filter(|&&t| t!=0).count() }
    pub fn is_solid(&self, x: usize, y: usize) -> bool { let t = self.tile_at(x,y); t!=0 && t!=0xFF }
}
fn rle(data: &[u8], pos: &mut usize, n: usize) -> Vec<u8> {
    let mut o = Vec::with_capacity(n);
    while o.len()<n && *pos+2<data.len() {
        let c=data[*pos]; let v1=data[*pos+1]; let v2=data[*pos+2]; *pos+=3;
        for _ in 0..((c>>4)+1) as usize { if o.len()<n { o.push(v1); } }
        for _ in 0..((c&0xF)+1) as usize { if o.len()<n { o.push(v2); } }
    }
    o.truncate(n); o
}
pub fn load_levels(path: &str) -> Vec<Level> {
    let d = std::fs::read(path).expect("levels");
    let mut ls = Vec::new(); let mut p = 0;
    while p+8 < d.len() {
        let w = u16::from_le_bytes([d[p],d[p+1]]) as usize;
        let h = u16::from_le_bytes([d[p+2],d[p+3]]) as usize;
        let bt = d[p+6]; let dp = d[p+7]; p+=8;
        if w==0||h==0||w>500||h>500 { break; }
        let n = w*h;
        let fg = rle(&d, &mut p, n);
        let bg = rle(&d, &mut p, n);
        let fg2 = fg.clone();
        let bg2 = bg.clone();
        let es = p;
        let mut found = false;
        for cp in p..d.len().min(p+2000).saturating_sub(8) {
            let cw = u16::from_le_bytes([d[cp],d[cp+1]]) as usize;
            let ch = u16::from_le_bytes([d[cp+2],d[cp+3]]) as usize;
            if cw==w && ch==h && cp>p+4 {
                ls.push(Level { width:w, height:h, bonus_target:bt, destruction_pct:dp, foreground:fg2, background:bg2, entity_data:d[es..cp].to_vec() });
                p = cp; found = true; break;
            }
        }
        if !found {
            ls.push(Level { width:w, height:h, bonus_target:bt, destruction_pct:dp, foreground:fg, background:bg, entity_data:d[es..d.len().min(es+200)].to_vec() });
            break;
        }
    }
    while ls.len() < 7 {
        if let Some(b) = ls.first() { let mut l = b.clone(); l.destruction_pct = (50+ls.len() as u8*5).min(90); l.bonus_target = ls.len() as u8; ls.push(l); } else { break; }
    }
    ls
}
#[derive(Clone, Debug)]
pub struct MonsterDef { pub data: [u8; 57] }
pub fn load_monster_defs(path: &str) -> Vec<MonsterDef> {
    let d = std::fs::read(path).expect("mst");
    let mut ds = Vec::new();
    for i in 0..7 { let s=i*57; if s+57<=d.len() { let mut m = MonsterDef{data:[0;57]}; m.data.copy_from_slice(&d[s..s+57]); ds.push(m); } }
    while ds.len() < 7 { ds.push(MonsterDef{data:[0;57]}); }
    ds
}
