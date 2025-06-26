use core::marker::PhantomData;
use ratatui::backend::Backend;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{Frame, Terminal};

#[derive(Default)]
pub struct BootloaderApp<B> {
    exit: bool,
    phantom: PhantomData<B>,
}

impl<B> BootloaderApp<B>
where
    B: Backend,
{
    pub fn run(&mut self, terminal: &mut Terminal<B>) -> Result<(), B::Error> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events();
        }

        Ok(())
    }

    fn calculate_layout(&self, area: Rect) -> (Rect, Rect) {
        let layout = Layout::vertical([Constraint::Min(2), Constraint::Percentage(100)]);
        let [top, bottom] = layout.areas(area);

        (top, bottom)
    }

    fn render(&mut self, frame: &mut Frame) {
        let (title, main) = self.calculate_layout(frame.area());
        self.render_title(frame, title);
        
        frame.render_widget(self, main);
    }
    
    fn render_title(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new("Dyspxcrypt Bootloader"), area);
    }

    fn handle_events(&mut self) {}
}

impl<B> Widget for &mut BootloaderApp<B>
where
    B: Backend,
{
    fn render(self, _: Rect, _: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
