use makepad_widgets::*;
   
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import test_1::demofiletree::*;
    import test_1::my_widget::*;
    import makepad_draw::shader::std::*;

    COLOR_CONTAINER = (THEME_COLOR_D_1)
    COLOR_ACCENT = (THEME_COLOR_MAKEPAD)

    DEMO_COLOR_1 = #8f0
    DEMO_COLOR_2 = #0f8
    DEMO_COLOR_3 = #80f

    ZooTitle = <View> {
        width: Fill, height: Fit,
        flow: Down,
        align: { x: 0.0, y: 0.5},
        margin: <THEME_MSPACE_3> {},
        spacing: 10.,
        show_bg: false,
        title = <H2> { text: "Cubicle Test Page" }
    }

    ZooHeader = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: (THEME_SPACE_1),
         margin: <THEME_MSPACE_H_3> {}
        divider = <Hr> { }
        title = <H3> { text: "Header" }
    }

    ZooGroup = <RoundedView> {
        height: Fit, width: Fill,
        flow: Right,
        align: { x: 0.0, y: 0.5},
        margin: 0.,
        show_bg: false;
        draw_bg: { color: (COLOR_CONTAINER) }
    }

    ZooDesc = <P> { text: "" }

    ZooBlock = <RoundedView> {
        width: 50., height: 50.
        margin: 0.,
        spacing: 0.,

        show_bg: true;
        draw_bg: {
            fn get_color(self) -> vec4 {
                return mix(self.color, self.color*0.5, self.pos.y);
            }
            radius: (THEME_CONTAINER_CORNER_RADIUS)
        }
    }
    
    App = {{App}} {

        ui: <Window>{ 
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    // test
                    //return mix(#7, #3, self.pos.y);
                    let uv = self.pos - 0.5;
                    let uv0 = uv;
                    let finalColor = vec3(0.0);

                    let i = 0;
                    //for _i in 0..4 { // you cannot refer to _i inside the for loop; use i instead
                    
                    for number in 1..4 {
                        uv = fract(uv * -1.5) - 0.5;
                        let d = length(uv) * exp(-length(uv0));
                        let col = Pal::iq2(length(uv0) + float(i) * .4 + self.time * .4);
                        d = sin(d*8. + self.time) / 8.;
                        d = abs(d);
                        d = pow(0.01 / d, 1.2);
                        finalColor += col * d;
                        i = i+1;
                    }
                    
                    return vec4(finalColor ,1);
                }
            }
            
            body = <ScrollXYView>{
                flow: Down,
                spacing:10,
                align: {
                    x: 0.5,
                    y: 0.5
                },

                <ZooTitle> {}

                <ZooHeader> {
                    title = {text: "Shader"}
                    <ZooDesc> {
                        text: "Shader Demo."
                    }
                    <MyWidget> {
                    //<RoundedView dx:395.4 dy:2855.5 dw:390.0 dh:137.4> {
                        width: 300, height: 300,
                        draw: {
                            // this example shader is ported from kishimisu's tutorial
                            fn pixel(self) -> vec4 {
                                let uv = self.pos - 0.5;
                                let uv0 = uv;
                                let finalColor = vec3(0.0);

                                let i = 0;
                                //for _i in 0..4 { // you cannot refer to _i inside the for loop; use i instead
                                
                                for number in 1..4 {
                                    uv = fract(uv * -1.5) - 0.5;
                                    let d = length(uv) * exp(-length(uv0));
                                    let col = Pal::iq2(length(uv0) + float(i) * .4 + self.time * .4);
                                    d = sin(d*8. + self.time) / 8.;
                                    d = abs(d);
                                    d = pow(0.01 / d, 1.2);
                                    finalColor += col * d;
                                    i = i+1;
                                }
                                
                                return vec4(finalColor ,1);
                            }
                        }
                    }
                }

                <RoundedView dx:395.4 dy:2855.5 dw:390.0 dh:137.4> {
                    width: Fill,
                    height: 120
                    draw_bg: {
                        color: #x44,
                        instance color2: #x0,
            
                        instance attack: 0.05
                        instance hold: 0.0
                        instance decay: 0.2
                        instance sustain: 0.5
                        instance release: 0.2
            
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size); //mod (self.pos * self.rect_size, 15))
                            let base_color = mix(self.color, self.color2, pow(length((self.pos - vec2(0.5, 0.5)) * 1.2), 2.0));
                            let darker = base_color * 0.85;
                            let pos = self.pos * self.rect_size;
                            sdf.clear(mix(base_color, darker, pow(abs(sin(pos.x * 0.5)), 24) + pow(abs(sin(pos.y * 0.5)), 32.0))); // Grid
                            sdf.rect(1.0, 1.0, 16, 16)
                            sdf.stroke(darker, 1)
                            let pad_b = 8
                            let pad_s = 8
                            let width = self.rect_size.x - 2 * pad_s
                            let height = self.rect_size.y - 2 * pad_b
                            let total = self.attack + self.decay + self.release + 0.5 + self.hold
                            let sustain = self.rect_size.y - pad_b - height * self.sustain;
                            sdf.pos = self.pos * self.rect_size;
                            sdf.move_to(pad_s, self.rect_size.y - pad_b)
                            sdf.line_to(pad_s + width * (self.attack / total), pad_b)
                            sdf.line_to(pad_s + width * ((self.attack + self.hold) / total), pad_b)
                            sdf.line_to(pad_s + width * ((self.attack + self.decay + self.hold) / total), sustain)
                            sdf.line_to(pad_s + width * (1.0 - self.release / total), sustain)
                            sdf.line_to(pad_s + width, self.rect_size.y - pad_b)
                            sdf.stroke_keep(#xFFC49910, 8.0);
                            sdf.stroke_keep(#xFFC49910, 6.0);
                            sdf.stroke_keep(#xFFC49920, 4.0);
                            sdf.stroke_keep(#xFFC49980, 2.0);
                            sdf.stroke(#xFFFFFFFF, 1.0);
                            return sdf.result
                        }
                    }
                }

                <ZooHeader> {
                    title = {text: "Intro"}
                    <ZooDesc> {
                        text: "Intro."
                    }
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        <P> { text: "- Shader-based: what does that mean for how things work." }
                        <P> { text: "- Inheritance mechanisms in the DSL." }
                        <P> { text: "- Introduction to the layout system." }
                        <P> { text: "- Base theme parameters." }
                        <P> { text: "- Typographic system. Base font-size and contrast." }
                        <P> { text: "- Space constants to control denseness of the design." }
                        <P> { text: "- Transparency mechanism of the widgets. Nesting for structure." }
                    }
                }

                <ZooHeader> {
                    title = {text: "Control Heights & Text Baselines"}
                    <ZooDesc> {
                        text: "Control heights and text baselines"
                    }
                    <View> {
                        width: Fill, height: Fit,
                        align: { x: 0., y: 0.}
                        flow: Right,
                        spacing: (THEME_SPACE_2)
                        <P> { text: "TestButton", width: Fit}
                        <LinkLabel> { text: "TestButton", width: Fit}
                        <CheckBox> { text: "TestButton"}
                        <CheckBoxToggle> { text: "TestButton"}
                        <ButtonFlat> { text: "TestButton"}
                        <Button> { text: "TestButton"}
                        <TextInput> { text: "TestButton"}
                        <DropDown> { }
                        <Slider> { text: "TestButton"}
                        <SliderBig> { text: "TestButton"}
                        // <RadioButton> { }
                        // <RadioButtonTextual> { }
                        // <RadioButtonTab> { }
                    }
                }

                <ZooDesc> {text:"This view is bigger on the inside"}
                <View> {
                    width: 150, height: 150,
                    flow: Right,
                    padding: 10.
                    spacing: 10.

                    show_bg: true,
                    draw_bg: { color: (COLOR_CONTAINER) }
                    scroll_bars: <ScrollBars> {}

                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        show_bg: false,
                        spacing: 10
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                    }

                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        show_bg: false,
                        spacing: 10
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                    }

                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        show_bg: false,
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                    }

                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        show_bg: false,
                        spacing: 10
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                    }

                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        show_bg: false,
                        spacing: 10
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                    }
                }

                <ZooHeader> {
                    title = {text:"<Label>"}
                    <ZooDesc> { text:"Default single line textbox" }
                    <ZooGroup> { <Label> { text: "This is a small line of text" } }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                color: (COLOR_ACCENT)
                                text_style: {
                                    font_size: 20,
                                }
                            },
                            text: "You can style text using colors and fonts"
                        }
                    }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                fn get_color(self) ->vec4{
                                    return mix((COLOR_ACCENT), (THEME_COLOR_U_HIDDEN), self.pos.x)
                                }
                                color: (THEME_COLOR_MAKEPAD)
                                text_style: {
                                    font_size: 40.,
                                }
                            },
                            text: "OR EVEN SOME PIXELSHADERS"
                        }
                    }
                }

                <ZooHeader> {
                    title = { text:"<Slider>" }
                    <ZooDesc> { text:"A parameter dragger" }
                    <ZooGroup> {
                        width: Fill, height: Fit,
                        flow: Right,
                        align: { x: 0., y: 0.}
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <Slider> { text: "Default" }
                            <Slider> { text: "label_align", label_align: { x: 0.5, y: 0. } }
                            <Slider> { text: "min/max", min: 0., max: 100. }
                            <Slider> { text: "precision", precision: 20 }
                            <Slider> { text: "step", step: 0.1 }
                        }
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <SliderBig> { text: "Default" }
                            <SliderBig> { text: "label_align", label_align: { x: 0.5, y: 0. } }
                            <SliderBig> { text: "min/max", min: 0., max: 100. }
                            <SliderBig> { text: "precision", precision: 20 }
                            <SliderBig> { text: "step", step: 0.1 }
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<DropDown>"}
                    <ZooDesc> {text:"DropDown control. This control currently needs to be databound which needs some plumbing. In this sample there is a binding context struct in the main app struct - which gets bound on app start - and updated during handle_actions."}
                    <ZooGroup> {
                        dropdown = <DropDown> {
                            labels: ["Value One", "Value Two", "Thrice", "Fourth Value", "Option E", "Hexagons"],
                            values: [ValueOne, ValueTwo, Thrice, FourthValue, OptionE, Hexagons]
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<FileTree>"}
                    <ZooDesc> {text:"File Tree"}
                    <ZooGroup> {
                        <DemoFileTree> { file_tree:{ height: 400. } }
                    }
                }

                <ZooHeader> {
                    title = { text:"<FoldHeader>" }
                    <ZooDesc> { text:"This widget allows you to have a header with a foldbutton (has to be named fold_button for the magic to work)" }
                    <ZooGroup> {
                        thefoldheader= <FoldHeader> {
                            header: <View> {
                                height: Fit
                                align: {x: 0., y: 0.5}
                                fold_button = <FoldButton> {} <P> {text: "Fold me!"}
                            }
                            body: <View> {
                                width: Fill, height: Fit
                                show_bg: false,
                                padding: 5.0,
                                <P> { text:"This is the body that can be folded away" }
                            }
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Html>"}
                    <ZooDesc> {text:"HTML Widget"}
                    <ZooGroup> {
                        <Html> {
                            width:Fill, height:Fit,
                            body:"<H1>H1 Headline</H1><H2>H2 Headline</H2><H3>H3 Headline</H3><H4>H4 Headline</H4><H5>H5 Headline</H5><H6>H6 Headline</H6>This is <b>bold</b>&nbsp;and <i>italic text</i>.<sep><b><i>Bold italic</i></b>, <u>underlined</u>, and <s>strike through</s> text. <p>This is a paragraph</p> <code>A code block</code>. <br/> And this is a <a href='https://www.google.com/'>link</a><br/><ul><li>lorem</li><li>ipsum</li><li>dolor</li></ul><ol><li>lorem</li><li>ipsum</li><li>dolor</li></ol><br/> <blockquote>Blockquote</blockquote> <pre>pre</pre><sub>sub</sub><del>del</del>"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Markdown>"}
                    <ZooDesc> {text:"Markdown"}
                    <ZooGroup> {
                        <Markdown> {
                            width:Fill, height: Fit,
                            body:"# Headline 1 \n ## Headline 2 \n ### Headline 3 \n #### Headline 4 \n This is standard text with a  \n\n line break a short ~~strike through~~ demo.\n\n *Italic text* \n\n **Bold text** \n\n - Bullet\n - Another bullet\n\n - Third bullet\n\n 1. Numbered list Bullet\n 2. Another list entry\n\n 3. Third list entry\n\n `Monospaced text`\n\n> This is a quote.\n\nThis is `inline code`.\n\n ```code block```"
                        }
                    }
                }

                /*quad = <MyWidget> {
                    width: 300, height: 300
                    draw: {
                        // this example shader is ported from kishimisu's tutorial
                        fn pixel(self) -> vec4 {
                            let uv = self.pos - 0.5;
                            let uv0 = uv;
                            let finalColor = vec3(0.0);

                            let i = 0;
                            for _i in 0..4 { // you cannot refer to _i inside the for loop; use i instead
                                uv = fract(uv * -1.5) - 0.5;
                                let d = length(uv) * exp(-length(uv0));
                                let col = Pal::iq2(length(uv0) + float(i) * .4 + self.time * .4);
                                d = sin(d*8. + self.time) / 8.;
                                d = abs(d);
                                d = pow(0.01 / d, 1.2);
                                finalColor += col * d;
                                i = i+1;
                            }

                            return vec4(finalColor ,1);
                        }
                    }
                }*/

                button1 = <Button> {
                    text: "Hello world"
                    draw_text:{color:#f00}
                }
                input1 = <TextInput> {
                    width: 100, height: 30
                    text: "Click to count"
                }
                label1 = <Label> {
                    draw_text: {
                        color: #f
                    },
                    text: "Counter: 0"
                }
                <Html>{
                    // a = {
                    //     draw_text: {
                    //         // other blue hyperlink colors: #1a0dab, // #0969da  // #0c50d1, #x155EEF, // #0a84ff
                    //         // color: #1a0dab,
                    //     }
                    // }

                    Button = <Button> {
                        text: "Helloworld"
                    }  
                    body:"
                    
                    Normal <u>underlined html</u> <s>strike</s> text hello world <br/>
                    <ol>
                        <li>one in the list!!!!! </li>
                        <li>two</li>
                        <li>three
                            <ol>
                                <li>sub one</li>
                                <li>sub two</li>
                                <li>sub three
                                    <ol>
                                        <li>sub sub one</li>
                                        <li>sub sub two</li>
                                        <li>sub sub three</li>
                                    </ol>
                                </li>
                            </ol>
                        </li>
                    </ol>
                    inline <code>let x = 1.0;</code> code
                    <b>BOLD text</b>&nbsp;<i>italic</i><br/>
                    <sep/>
                    Testing a link: <a href=\"https://www.google.com\">Click to Google</a><br/>
                    Next line normal text button:<Button>Hi</Button><br/>
                    <blockquote>block<b>quote</b><br/><blockquote>blockquote</blockquote><br/>
                    Next line <br/>
                    <sep/>
                    </blockquote><b><i>Bold italic</i><br/>
                    <sep/></br>
                    <pre>this is a preformatted code block</pre>
                    "
                }
                <Markdown>{
                    
                    body:"
                    # MD H1 
                    ## H2 **Bold** *italic*
                    
                    
                    1. aitem
                    1. item
                      1. item  
                      1. test  
                    4. item               
                                          
                    > block
                    > next
                    >> hi
                    continuation
                    
                    [link](https://image)
                    ![image](https://link)
                    Normal
                    Next line
                    
                    ---
                    ~~single newline~~ becomes space
                    *hello*hello world
                    
                        inline code
                        more inline code
                    Double newline
                    `inline code` text after
                    ```
                    let x = 10
                    let y = 10
                    ```
                    *italic* **Bold** normal _italic_ __bold__ ***Bolditalic*** normal
                    123
                    "
                }
            }
        }
    }
}  
              
app_main!(App); 
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] counter: usize,
 }

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        //  crate::demofiletree::live_design(cx);
        crate::my_widget::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        if self.ui.button(id!(button1)).clicked(&actions) {
            log!("BUTTON CLICKED {}", self.counter); 
            self.counter += 1;
            let label = self.ui.label(id!(label1));
            label.set_text_and_redraw(cx,&format!("Counter: {}", self.counter));
            //log!("TOTAL : {}",TrackingHeap.total());
            
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 
/*


// This is our custom allocator!
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicU64, Ordering},
};

pub struct TrackingHeapWrap{
    count: AtomicU64,
    total: AtomicU64,
}

impl TrackingHeapWrap {
    // A const initializer that starts the count at 0.
    pub const fn new() -> Self {
        Self{
            count: AtomicU64::new(0),
            total: AtomicU64::new(0)
        }
    }
    
    // Returns the current count.
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
    
    pub fn total(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for TrackingHeapWrap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Pass everything to System.
        self.count.fetch_add(1, Ordering::Relaxed); 
        self.total.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }
        
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.count.fetch_sub(1, Ordering::Relaxed); 
        self.total.fetch_sub(layout.size() as u64, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}

// Register our custom allocator.
#[global_allocator]
static TrackingHeap: TrackingHeapWrap = TrackingHeapWrap::new();*/