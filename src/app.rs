use eframe::egui;

use crate::components::next_page::NextPage;
use crate::pages::all::*;
use crate::traits::{Component, Page, PageV};

pub struct App {
    pages: Vec<PageV>,
    next_page: NextPage,
}

impl Default for App {
    fn default() -> Self {
        let pages: Vec<PageV> = vec![Box::new(Setup::new())];
        let mut next_page = NextPage::new();
        next_page.pages_len = Some(pages.len());

        Self { pages, next_page }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            let page = &mut self.pages[self.next_page.page_id.lock().unwrap().clone()];
            page.update(ctx);
            page.draw(ui, ctx);

            self.next_page.draw(ui, ctx);
        });
    }
}
