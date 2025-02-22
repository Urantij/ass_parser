///! # AssParser
///! 
///! [ass_parser] is a crate to parse .ass (Advanced SubStation Alpha) files. which is a subtitle file for creating and displaying subtitles in video files. It is widely used due to it's complex text formatting, positioning and styling. The Advanced SubStation Alpha is a successor
///! to the SubStation Alpha .ssa file.
///! 
///! ## Installation
///! 
///! Add `ass_parser` as a dependency to your cargo.toml:
///! 
///!  ```shell
///!  cargo add ass_parser
///!  ```
///! # Introduction
///! 
///! AssParser is based on the principle of easy to read write and modify `.ass` files. This is the first version of `ass_parser`and now currently only have the features to modify `.ass` file.
///! 
///! # Example
///! 
/// Creating a simple `Advanced SubStation Alpha` `(.ass)` file with default values!
///
/// ```rust
/// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
/// use hex_color::HexColor;
/// 
/// fn main() {
///     let mut ass_file = AssFile::new();
///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
/// 
///     ass_file.components.script
///         .set_script(ScriptInfo::default());
/// 
///     ass_file.components.v4
///         .set_v4(V4Format::default())
///         .set_primarycolour(&hexcolor);
/// 
///     ass_file.components.events
///         .set_events(Events::default());
/// 
///     AssFile::save_file(&ass_file, "new_subtitles.ass")
/// 
/// }
/// 
/// ```
/// Here we create an .ass file with default values and When you open the .ass file you can see the
/// following content.
/// ```
/// ScriptType: v4.00+
/// PlayResX: 384
/// PlayResY: 288
/// ScaledBorderAndShadow: yes
/// YCbCr Matrix: None
/// 
/// 
/// [V4+ Styles]
/// Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
/// Style: Default,Arial,16,&H00ff,&Hffffff,&H0,&H0,0,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1
/// 
/// 
/// [Events]
/// Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
/// Dialogue: 0,0:00:00.00,0:00:01.00,Default,,0,0,0,,Hello Friend
/// ```
///
/// # Add Dialogues
///
/// ```rust
/// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions, Dialogue};
/// use ass_parser::IndexNotFound;
/// use hex_color::HexColor;
/// 
/// fn main() -> Result<(), IndexNotFound>{
///     let mut ass_file = AssFile::new();
///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
/// 
///     let first_dialogue = Dialogue::default()
///         .set_text("Hello There!")
///         .set_start("0:00:00.10")
///         .set_end("0:00:00.50");
/// 
///     let second_dialogue = Dialogue::default()
///         .set_text("Hello Friend!")
///         .set_start("00:00.50")
///         .set_end("00:00.58");
/// 
///     let third_dialogue = Dialogue::default()
///         .set_text("Hello World!!")
///         .set_start("0:00:00.58")
///         .set_end("0:00:01.01");
/// 
///     let events = Events::new()
///         .add_first_dialogue(first_dialogue)?
///         .add_dialogue(second_dialogue)
///         .add_dialogue(third_dialogue)
///         .create();
/// 
/// 
///     ass_file.components.script
///         .set_script(ScriptInfo::default())
///         .set_scripttype("FFMPEG");
/// 
///     ass_file.components.v4
///         .set_v4(V4Format::default())
///         .set_primarycolour(&hexcolor);
/// 
///     ass_file.components.events
///         .set_events(events);
/// 
///     AssFile::save_file(&ass_file, "new_subtitles.ass");
/// 
///     Ok(())
/// 
/// }
/// ```
///
/// # Add Colors to Subtitles.
///
/// You can add individual colors to each subtitles using the `.set_colour()` function. This
/// function takes HexColor. Make sure that you are using rand + std features to generate random colors via rand out of the box.
///
/// ```rust
///
///let random_color:HexColor = rand::random();
///
///let dialogue = Dialogue::default()
///    .set_start(&start)
///    .set_end(&end)
///    .set_text(&text)
///    .set_colour(random_color);
///
///event.add_dialogue(dialogue);
/// ```
///
/// # Modify Existing ASS files.
///
/// Use the `from_file` function of AssFile to modify and change the contents or appearance. 
///
/// ``` rust
/// use ass_parser::{AssFile, Dialogue, AssFileOptions};
/// use hex_color::HexColor;
/// 
/// fn main() -> Result<(), std::io::Error>{
///     let mut ass_file = AssFile::from_file("subtitles.ass")?;
///     let dialogue = Dialogue::default()
///         .set_text("Hello Friend!");
///     let primary_color = AssFileOptions::get_ass_color(HexColor::RED);
/// 
/// 
///     ass_file.components.v4
///         .set_primarycolour(&primary_color);
///         
///     ass_file.components.events
///         .add_dialogue(dialogue);
/// 
///     AssFile::save_file(&ass_file, "new_subtitles.ass");
/// 
///     Ok(())
/// }
/// ```
///
/// # Added Support for SubRip files.
///
/// Now you can load `.srt` files and convert them to `.ass` files and even modify them on the
/// process too. Here is an example from the `examples` directory.
///
/// In this example we load an SubRip file (`RapGod.srt`) and extract each subtitle from it and
/// modify them by adding random colors to each subtitle. Then finally converting it to a `.ass`
/// file and saving it.
///
/// ```rust
/// use hex_color::HexColor;
/// use ass_parser::{AssFile, AssFileOptions};
/// use ass_parser::{ScriptInfo, V4Format, Events, Dialogue};
/// use rand;
/// 
/// fn main() {
///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
///     let srt_file = AssFile::from_srt("RapGod.srt");
///     let mut ass_file = AssFile::new();
///     let mut event = Events::default();
/// 
///     for srt_seg in srt_file.iter() {
///         let start = &srt_seg.start;
///         let end = &srt_seg.end;
///         let text = &srt_seg.text;
/// 
///         let random_color:HexColor = rand::random();
/// 
///         let dialogue = Dialogue::default()
///             .set_start(&start)
///             .set_end(&end)
///             .set_text(&text)
///             .set_colour(random_color);
/// 
///         event.add_dialogue(dialogue);
///     }
///     
/// 
///     ass_file.components.script
///         .set_script(ScriptInfo::default());
/// 
/// 
/// 
///     ass_file.components.v4
///         .set_v4(V4Format::default())
///         .set_primarycolour(&hexcolor);
///     ass_file.components.events
///         .set_events(event);
/// 
///     AssFile::save_file(&ass_file, "new_subtitle.ass");
/// }
/// ```
///
///
/// ## This will generate an ASS file which would be similiar to this
///
/// ```
///ScriptType: FFMPEG
///PlayResX: 384
///PlayResY: 288
///ScaledBorderAndShadow: yes
///YCbCr Matrix: None
///
///
///[V4+ Styles]
///Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
///Style: Default,Arial,16,&H0ffff,&Hffffff,&H0,&H0,0,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1
///
///
///[Events]
///Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
///Dialogue: 0,0:00:00.10,0:00:00.50,Default,,0,0,0,,Hello There!
///Dialogue: 0,00:00.50,00:00.58,Default,,0,0,0,,Hello Friend!
///Dialogue: 0,0:00:00.58,0:00:01.01,Default,,0,0,0,,Hello World!!
/// ```
/// # Events can also be created like this
///
///
///```rust
///let first_dialogue = Dialogue::default()
///   .set_start("0:00:00.10")
///   .set_end("0:00:00.50");
///
///let second_dialogue = Dialogue::default()
///   .set_start("00:00.50")
///   .set_end("00:00.58");
///
///let third_dialogue = Dialogue::default()
///   .set_start("0:00:00.58")
///   .set_end("0:00:01.01");
///
///let events = Events::new()
///   .add_first_dialogue(first_dialogue)?
///   .add_dialogue(second_dialogue)
///   .add_dialogue(third_dialogue)
///   .create();
/// ```
///
/// You can burn this subtitle file to a video or use any video player to select a video file along
/// with this subtitle file.
///
/// # Using [FFmpeg] to burn the video with the subtitles file.
///
/// You will first have to download and install [FFmpeg] on your system to try this. Once you have
/// downloaded you can use the following command to burn the video file `video.avi` and the
/// generated subtitle file `new_subtitles.ass` to a single output video file `output.avi`
///
/// ```shell
/// ffmpeg -i video.avi -vf "ass=new_subtitles.ass" output.avi
/// ```
///! 
///! [FFmpeg]: https://www.ffmpeg.org/about.html
///! [ass_parser]: https://github.com/Aavtic/ass_parser


use hex_color::HexColor;
use std::{fs, io::Read};
use std::io::{Seek, Write};
use std::ops::Deref;
use std::fmt;
use std::iter::Iterator;

mod parser;

type SrtData = parser::SrtContent;

const SCRIPT_HEADER:&str = "[Script Info]";
const SCRIPT_TYPE:&str = "ScriptType: ";
const SCRIPT_PLAYRESX:&str = "PlayResX: ";
const SCRIPT_PLAYRESY:&str = "PlayResY: ";
const SCRIPT_SCALEDBORDERANDSHADOW:&str =  "ScaledBorderAndShadow: ";
const SCRIPT_YCBCR_MATRIX:&str =  "YCbCr Matrix: ";
const V4_HEADER:&str = "[V4+ Styles]";
const V4_STYLE_HEAD:&str = "Style: ";
const EVENTS_HEADER:&str = "[Events]";
const EVENT_HEAD:&str = "Dialogue: ";


type Result<T> = std::result::Result<T, IndexNotFound>;

#[derive(Debug, Clone)]
pub struct IndexNotFound;

impl std::fmt::Display for IndexNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The Index is not found on Dialogues.")
    }
}


/// The First part of any Advanced SubStation Alpha file is `Script Info`.
/// This holds necessary information which include the version the resolution of subtitles etc of
/// the `.ass` file.

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptInfo {
    scripttype: Option<String>,
    playresx: Option<String>,
    playresy: Option<String>,
    scaledborderandshadow: Option<String>,
    ycbcr_matrix: Option<String>,
}

impl ScriptInfo {
    fn get_key_values(&self) -> Vec<[&str; 2]> {
        let mut values = Vec::new();

        if let Some(value) = &self.scripttype {
            values.push([SCRIPT_TYPE, value])
        }
        if let Some(value) = &self.playresx {
            values.push([SCRIPT_PLAYRESX, value])
        }
        if let Some(value) = &self.playresy{
            values.push([SCRIPT_PLAYRESY, value])
        }
        if let Some(value) = &self.scaledborderandshadow {
            values.push([SCRIPT_SCALEDBORDERANDSHADOW, value])
        }
        if let Some(value) = &self.ycbcr_matrix {
            values.push([SCRIPT_YCBCR_MATRIX, value])
        }
        values
    }
}

impl ScriptInfo {
    fn new() -> Self {
        Self {
    		scripttype: None,
    		playresx: None,
    		playresy: None,
    		scaledborderandshadow: None,
    		ycbcr_matrix: None,
        }
    }

    pub fn set_script(&mut self, script: ScriptInfo) -> &mut ScriptInfo {
        *self = script;
        self
    }
}

impl Default for ScriptInfo {
    fn default() -> ScriptInfo {
        ScriptInfo {
    		scripttype: Some("v4.00+".to_string()),
    		playresx: Some("384".to_string()),
    		playresy: Some("288".to_string()),
    		scaledborderandshadow: Some("yes".to_string()),
    		ycbcr_matrix: Some("None".to_string()),
        }
    }
}

impl ScriptInfo {
    /// After creating the `AssFile` set the scripttype of the .ass file.
    /// If you want to specify any, the default ScriptType from the original `.ass` file will be
    /// used.
    /// This is the SSA script format version eg. "V4.00". It is used by SSA to give a warning if
    /// you are using a version of SSA older than the version that created the script.
    /// ASS version is “V4.00+”.
    pub fn set_scripttype(&mut self, value: &str) -> &mut Self {
		self.scripttype = Some(value.to_string());
		self
	}
    /// After creating the `AssFile` set the playresx of the .ass file.
    ///
    /// This is the height of the screen used by the script's author(s) when playing the script. SSA v4 will automatically select the nearest enabled setting, if you are using Directdraw playback.
    ///
    /// If you don't want to specify any, the default playresx from the original `.ass` file will be
    /// used.
    pub fn set_playresx(&mut self, value: &str) -> &mut Self {
		self.playresx = Some(value.to_string());
		self
	}
    /// After creating the `AssFile` set the playresy of the .ass file.
    ///
    /// This is the height of the screen used by the script's author(s) when playing the script. SSA v4 will automatically select the nearest enabled setting, if you are using Directdraw playback.
    ///
    /// If you want to specify any, the default playresy from the original `.ass` file will be
    /// used.
    /// 
    pub fn set_playresy(&mut self, value: &str) -> &mut Self {
		self.playresy = Some(value.to_string());
		self
	}
    /// After creating the `AssFile` set the scaledborderandshadow of the .ass file.
    /// If you want to specify any, the default scaledborderandshadowfrom the original `.ass` file will be
    /// used.
    pub fn set_scaledborderandshadow(&mut self, value: &str) -> &mut Self {
		self.scaledborderandshadow = Some(value.to_string());
		self
	}
    /// After creating the `AssFile` set the ycbcr_matrix( of the .ass file.
    /// If you want to specify any, the default ycbcr_matrix from the original `.ass` file will be
    /// used.
    pub fn set_ycbcr_matrix(&mut self, value: &str) -> &mut Self {
		self.ycbcr_matrix = Some(value.to_string());
		self
	}
}


/// # V4Format
///
/// The Second part of any Advanced SubStation Alpha file is `V4Format`.
/// This is the part which has fields separated by comma which specify the format, styling,
/// encoding colors and many other important parts of the the `.ass` file.

#[derive(Debug, PartialEq, Clone)]
pub struct V4Format {
    name: Option<String>,
    fontname: Option<String>,
    fontsize: Option<String>,
    primarycolour: Option<String>,
    secondarycolour: Option<String>,
    outlinecolour: Option<String>,
    backcolour: Option<String>,
    bold: Option<String>,
    italic: Option<String>,
    underline: Option<String>,
    strikeout: Option<String>,
    scalex: Option<String>,
    scaley: Option<String>,
    spacing: Option<String>,
    angle: Option<String>,
    borderstyle: Option<String>,
    outline: Option<String>,
    shadow: Option<String>,
    alignment: Option<String>,
    marginl: Option<String>,
    marginr: Option<String>,
    marginv: Option<String>,
    encoding: Option<String>,
}

impl V4Format {
    fn new() -> V4Format {
        Self {
            name: None,
            fontname: None,
            fontsize: None,
            primarycolour: None,
            secondarycolour: None,
            outlinecolour: None,
            backcolour: None,
            bold: None,
            italic: None,
            underline: None,
            strikeout: None,
            scalex: None,
            scaley: None,
            spacing: None,
            angle: None,
            borderstyle: None,
            outline: None,
            shadow: None,
            alignment: None,
            marginl: None,
            marginr: None,
            marginv: None,
            encoding: None,
        }
    }
}

impl Default for V4Format {
    /// V4 Set with the common '`Default`' Format for `Advanced SubStation Alpha`.
    fn default() -> V4Format {
        V4Format {
        name: Some("Default".to_string()),
        fontname: Some("Arial".to_string()),
        fontsize: Some("16".to_string()),
        primarycolour: Some("&Hffffff".to_string()),
        secondarycolour: Some("&Hffffff".to_string()),
        outlinecolour:Some("&H0".to_string()),
        backcolour: Some("&H0".to_string()),
        bold: Some("0".to_string()),
        italic: Some("0".to_string()),
        underline: Some("0".to_string()),
        strikeout: Some("0".to_string()),
        scalex: Some("100".to_string()),
        scaley: Some("100".to_string()),
        spacing: Some("0".to_string()),
        angle:Some("0".to_string()),
        borderstyle:Some("1".to_string()),
        outline: Some("1".to_string()),
        shadow: Some("0".to_string()),
        alignment: Some("2".to_string()),
        marginl:Some("10".to_string()),
        marginr: Some("10".to_string()),
        marginv: Some("10".to_string()),
        encoding: Some("1".to_string()),
        }
    }
}

impl V4Format {
    /// Set V4 from a V4 Struct.
    pub fn set_v4(&mut self, v4: V4Format) -> &mut V4Format {
        *self = v4;
        self
    }
    fn get_array(&self) -> [&Option<String>; 23] {
        [
            &self.name,
            &self.fontname,
            &self.fontsize,
            &self.primarycolour,
            &self.secondarycolour,
            &self.outlinecolour,
            &self.backcolour,
            &self.bold,
            &self.italic,
            &self.underline,
            &self.strikeout,
            &self.scalex,
            &self.scaley,
            &self.spacing,
            &self.angle,
            &self.borderstyle,
            &self.outline,
            &self.shadow,
            &self.alignment,
            &self.marginl,
            &self.marginr,
            &self.marginv,
            &self.encoding,
            ]
    }

}

impl V4Format {
    // Ik this looks crazy. but what do?
    /// set the name for the V4 field.
    /// The name of the Style. Case sensitive. Cannot include commas
	pub fn set_name(&mut self,
                    value: &str) -> &mut Self{
        self.name = Some(value.to_string());
        self
	}
    /// set the fontname for the V4 field.
    /// The fontname as used by Windows. Case-sensitive.
        pub fn set_fontname(&mut self,
                        value: &str) -> &mut Self{
        self.fontname = Some(value.to_string());
        self
	}
    /// set the fontsize for the V4 field.
	pub fn set_fontsize(&mut self,
                        value: &str) -> &mut Self{
        self.fontsize = Some(value.to_string());
        self
	}
    /// set the primarycolour for the V4 field.
    /// ```rust
    /// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
    /// use hex_color::HexColor;
    /// 
    /// fn main() {
    ///     let mut ass_file = AssFile::new();
    ///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
    /// 
    ///     ass_file.components.script
    ///         .set_script(ScriptInfo::default());
    /// 
    ///     ass_file.components.v4
    ///         .set_v4(V4Format::default())
    ///         .set_primarycolour(&hexcolor);
    /// 
    ///     ass_file.components.events
    ///         .set_events(Events::default());
    /// 
    ///     AssFile::save_file(&ass_file, "new_subtitles.ass")
    /// }
    /// ```
	pub fn set_primarycolour(&mut self,
                             value: &str) -> &mut Self{
        self.primarycolour = Some(value.to_string());
        self
	}
    /// set the secondarycolour for the V4 field.
    /// ```rust
    /// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
    /// use hex_color::HexColor;
    /// 
    /// fn main() {
    ///     let mut ass_file = AssFile::new();
    ///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
    /// 
    ///     ass_file.components.script
    ///         .set_script(ScriptInfo::default());
    /// 
    ///     ass_file.components.v4
    ///         .set_v4(V4Format::default())
    ///         .set_secondarycolour(&hexcolor);
    /// 
    ///     ass_file.components.events
    ///         .set_events(Events::default());
    /// 
    ///     AssFile::save_file(&ass_file, "new_subtitles.ass")
    /// }
    /// ```
	pub fn set_secondarycolour(&mut self,
                               value: &str) -> &mut Self{
        self.secondarycolour = Some(value.to_string());
        self
	}
    /// set the outlinecolour for the V4 field.
    /// ```rust
    /// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
    /// use hex_color::HexColor;
    /// 
    /// fn main() {
    ///     let mut ass_file = AssFile::new();
    ///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
    /// 
    ///     ass_file.components.script
    ///         .set_script(ScriptInfo::default());
    /// 
    ///     ass_file.components.v4
    ///         .set_v4(V4Format::default())
    ///         .set_outlinecolour(&hexcolor);
    /// 
    ///     ass_file.components.events
    ///         .set_events(Events::default());
    /// 
    ///     AssFile::save_file(&ass_file, "new_subtitles.ass")
    /// }
    /// ```
	pub fn set_outlinecolour(&mut self,
                             value: &str) -> &mut Self{
        self.outlinecolour = Some(value.to_string());
        self
	}
    /// set the backcolour for the V4 field.
    /// ```rust
    /// use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
    /// use hex_color::HexColor;
    /// 
    /// fn main() {
    ///     let mut ass_file = AssFile::new();
    ///     let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
    /// 
    ///     ass_file.components.script
    ///         .set_script(ScriptInfo::default());
    /// 
    ///     ass_file.components.v4
    ///         .set_v4(V4Format::default())
    ///         .set_backcolour(&hexcolor);
    /// 
    ///     ass_file.components.events
    ///         .set_events(Events::default());
    /// 
    ///     AssFile::save_file(&ass_file, "new_subtitles.ass")
    /// }
    /// ```
	pub fn set_backcolour(&mut self,
                          value: &str) -> &mut Self{
        self.backcolour = Some(value.to_string());
        self
	}
    /// set the bold for the V4 field.
    /// This defines whether text is bold (true) or not (false). -1 is True, 0 is False. This is independant of the Italic attribute - you can have have text which is both bold and italic
	pub fn set_bold(&mut self,
                    value: &str) -> &mut Self{
        self.bold = Some(value.to_string());
        self
	}
    /// set the italic for the V4 field.
    /// This defines whether text is italic (true) or not (false). -1 is True, 0 is False. This is independant of the bold attribute - you can have have text which is both bold and italic.
	pub fn set_italic(&mut self,
                      value: &str) -> &mut Self{
        self.italic = Some(value.to_string());
        self
	}
    /// set the underline for the V4 field.
    ///  use either of [-1 or 0] where -1 is considered True and 0 is considered False.
	pub fn set_underline(&mut self,
                         value: &str) -> &mut Self{
        self.underline = Some(value.to_string());
        self
	}
    /// set the strikeout for the V4 field.
    ///  use either of [-1 or 0] where -1 is considered True and 0 is considered False.
	pub fn set_strikeout(&mut self,
                         value: &str) -> &mut Self{
        self.strikeout = Some(value.to_string());
        self
	}
    /// set the scalex for the V4 field.
    /// ScaleX. Modifies the width of the font. [percent]
	pub fn set_scalex(&mut self,
                      value: &str) -> &mut Self{
        self.scalex = Some(value.to_string());
        self
	}
    /// set the scaley for the V4 field.
    /// ScaleX. Modifies the height of the font. [percent]
	pub fn set_scaley(&mut self,
                      value: &str) -> &mut Self{
        self.scaley = Some(value.to_string());
        self
	}
    /// set the spacing for the V4 field.
    ///  Extra space between characters. [pixels]
	pub fn set_spacing(&mut self,
                       value: &str) -> &mut Self{
        self.spacing = Some(value.to_string());
        self
	}
    /// set the angle for the V4 field.
    /// The origin of the rotation is defined by the alignment. Can be a floating point number. [degrees]
	pub fn set_angle(&mut self,
                     value: &str) -> &mut Self{
        self.angle = Some(value.to_string());
        self
	}
    /// set the borderstyle for the V4 field.
    ///  pass either 1 or 3. where 1=Outline + drop shadow, 3=Opaque box.
	pub fn set_borderstyle(&mut self,
                           value: &str) -> &mut Self{
        self.borderstyle = Some(value.to_string());
        self
	}
    /// set the outline for the V4 field.
    /// If BorderStyle is 1,  then this specifies the width of the outline around the text, in pixels.
    /// Values may be 0, 1, 2, 3 or 4.
	pub fn set_outline(&mut self,
                       value: &str) -> &mut Self{
        self.outline = Some(value.to_string());
        self
	}
    /// set the shadow for the V4 field.
    /// If BorderStyle is 1,  then this specifies the depth of the drop shadow behind the text, in pixels. Values may be 0, 1, 2, 3 or 4. Drop shadow is always used in addition to an outline. 
	pub fn set_shadow(&mut self,
                      value: &str) -> &mut Self{
        self.shadow = Some(value.to_string());
        self
	}
    /// set the alignment for the V4 field.
    /// This sets how text is "justified" within the Left/Right onscreen margins, and also the vertical placing. Values may be 1=Left, 2=Centered, 3=Right. Add 4 to the value for a "Toptitle". Add 8 to the value for a "Midtitle".
    /// eg. 5 = left-justified toptitle
	pub fn set_alignment(&mut self,
                         value: &str) -> &mut Self{
        self.alignment = Some(value.to_string());
        self
	}
    /// set the marginl for the V4 field.
    /// This defines the Left Margin in pixels. It is the distance from the left-hand edge of the screen.The three onscreen margins (MarginL, MarginR, MarginV) define areas in which the subtitle text will be displayed.
	pub fn set_marginl(&mut self,
                       value: &str) -> &mut Self{
        self.marginl = Some(value.to_string());
        self
	}
    /// set the marginr for the V4 field.
    /// This defines the Right Margin in pixels. It is the distance from the right-hand edge of the screen. The three onscreen margins (MarginL, MarginR, MarginV) define areas in which the subtitle text will be displayed.
	pub fn set_marginr(&mut self,
                       value: &str) -> &mut Self{
        self.marginr = Some(value.to_string());
        self
	}
    /// set the marginv for the V4 field.
    /// This defines the vertical Left Margin in pixels.
    /// For a subtitle, it is the distance from the bottom of the screen.
    /// For a toptitle, it is the distance from the top of the screen.
    /// For a midtitle, the value is ignored - the text will be vertically centred.
	pub fn set_marginv(&mut self,
                       value: &str) -> &mut Self{
        self.marginv = Some(value.to_string());
        self
	}
    /// set the encoding for the V4 field.
    /// This specifies the font character set or encoding and on multi-lingual Windows installations it provides access to characters used in multiple than one languages. It is usually 0 (zero) for English (Western, ANSI) Windows.
	fn set_encoding(&mut self, value: &str) -> &mut Self{
        self.encoding = Some(value.to_string());
        self
	}
}


/// # Events
/// In `Advanced SubStation Alpha` Events is the core part of the subtitle file.
/// This contains Dialogues which can be subtitle text. and even Graphics.

#[derive(Debug, PartialEq, Clone)]
pub struct Events {
    pub dialogues: Dialogues,
}

impl Events {
    /// Returns a Clone of `Dialogues`
    /// You can then use this to access fields of `Dialogue`.
    /// ```rust 
    /// let dialogues = ass_file.events.get_dialogues().dialogues.clone();

    /// for dialogue in dialogues {
    ///     println!("layer: {:?}", &dialogue.get_layer());
    ///     println!("name: {:?}", &dialogue.get_name());
    ///     println!("end: {:?}", &dialogue.get_end());
    ///     println!("start: {:?}", &dialogue.get_start());
    ///     println!("text: {:?}", &dialogue.get_text());
    ///     println!("marginl: {:?}", &dialogue.get_marginl());
    ///     println!("marginr: {:?}", &dialogue.get_marginr());
    ///     println!("marginv: {:?}", &dialogue.get_marginv());
    ///     println!("style: {:?}", &dialogue.get_style());
    ///     println!("effect: {:?}", &dialogue.get_effect());
    ///     println!("colour: {:?}", &dialogue.get_colour());
    /// }
    /// ```
    pub fn get_dialogues(&self) -> Vec<Dialogue> {
        return self.dialogues.dialogues.clone();
    }
}

impl Events {
    /// Create a new instance of Event.
    /// This will have `None` for all the fields for EventFormat. 
    pub fn new() -> Events {
        let dialogue = Dialogue::new();
        Events {
            dialogues: 
                Dialogues {
                    dialogues: 
                        vec![
                            dialogue
                        ]
                }
        }
    }

   /// Create the final Event
   /// This simply consumes the mutable self and returns self.
   /// Call this function at the end of constructing an `Event`.
   ///
   /// # Example
   /// ```rust
   ///  let dialogue = Dialogue::default();
   ///  let events = Events::new()
   ///     .add_first_dialogue(dialogue.clone().set_text("Hello There!")).unwrap()
   ///     .add_dialogue(dialogue.clone().set_text("Hello Friend!"))
   ///     .add_n_dialogue(1, dialogue.clone().set_text("Hello Friend :)")).unwrap()
   ///     .add_last_dialogue(dialogue.set_text("Bye Friend.")).unwrap()
   ///     .create();
   /// ```

    pub fn create(&mut self) -> Self {
        self.clone()
    }

    /// Add a dialogue to the first of the `Events` Struct.
    /// # Example
    /// ```rust
   ///  let dialogue = Dialogue::default();
   ///  let events = Events::new()
   ///     .add_first_dialogue(dialogue.set_text("Hello There!")).unwrap();
   /// ```
   /// 
    pub fn add_first_dialogue(&mut self, dialogue: Dialogue) -> Result<&mut Self> {
        match self.dialogues.dialogues.first_mut() {
            Some(dlg) => {
                *dlg = dialogue;
                Ok(self)
            },
            None  => {
                Err(IndexNotFound)
            }
        }
    }

    /// Add a dialogue to the last of the `Events` Struct.
    /// # Example
    /// ```rust
   ///  let dialogue = Dialogue::default();
   ///  let events = Events::new()
   ///     .add_last_dialogue(dialogue.set_text("Hello There!")).unwrap();
   /// ```
    pub fn add_last_dialogue(&mut self, dialogue: Dialogue) -> Result<&mut Self> {
        match self.dialogues.dialogues.last_mut() {
            Some(dlg) => {
                *dlg = dialogue;
                Ok(self)
            },
            None  => {
                Err(IndexNotFound)
            }
        }
    }

    /// Add a dialogue to the nth position of the `Events` Struct.
    /// # Example
    /// ```rust
   ///  let dialogue = Dialogue::default();
   ///  let events = Events::new()
   ///     .add_n_dialogue(dialogue.set_text("Hello There!")).unwrap();
   /// ```
    pub fn add_n_dialogue(&mut self, n: usize, dialogue: Dialogue) -> Result<&mut Self> {
        match self.dialogues.dialogues.get_mut(n) {
            Some(dlg) => {
                *dlg = dialogue;
                Ok(self)
            },
            None  => {
                Err(IndexNotFound)
            }
        }
    }

    /// Add a dialogue to the end of the `Events` Struct.
    /// # Example
    /// ```rust
   ///  let dialogue = Dialogue::default();
   ///  let events = Events::new()
   ///     .add_n_dialogue(dialogue.set_text("Hello There!")).unwrap();
   /// ```
    pub fn add_dialogue(&mut self, dialogue: Dialogue) -> &mut Events {
        self.dialogues.dialogues.push(dialogue);
        self
    }
}

impl Default for Events {
    fn default() -> Events {
        Events {
            dialogues: 
                Dialogues {
                        dialogues: vec![
                        Dialogue {
                            event: EventFormat::default(),
                        }
                    ]
                }
        }
    }
}


impl Events {
    pub fn set_events(&mut self, events: Events) -> &mut Events {
        *self = events;
        self
    }
}

/// # Dialogues
/// This stores each `Dialogue: ` field in an `Advanced SubStation File`
#[derive(Debug, PartialEq,Clone)]
pub struct Dialogues {
    pub dialogues: Vec<Dialogue>
}

/// A single `Dialogue` which contain `event` which can be used to modify the state of a
/// `Dialogue`.
#[derive(Debug, PartialEq, Clone)]
pub struct Dialogue {
    event: EventFormat
}

#[derive(Debug, PartialEq,Clone)]
struct EventFormat {
    layer: Option<String>,
    start: Option<String>,
    end: Option<String>,
    style: Option<String>,
    name: Option<String>,
    marginl: Option<String>,
    marginr: Option<String>,
    marginv: Option<String>,
    effect: Option<String>,
    text: Option<String>,
    color: Option<String>
}

impl Default for EventFormat {
    fn default() -> EventFormat {
        EventFormat {
            layer: Some("0".to_string()),
            start: Some("0:00:00.00".to_string()),
            end: Some("0:00:00.00".to_string()),
            style: Some("Default".to_string()),
            name: Some("".to_string()),
            marginl: Some("0".to_string()),
            marginr: Some("0".to_string()),
            marginv: Some("0".to_string()),
            effect: Some("".to_string()),
            text: None,
            color: None,
        }
    }
}

impl Dialogue {
    pub fn new() -> Self {
        Self {
            event: EventFormat {
                layer: None,
                start: None,
                end: None,
                style: None,
                name: None,
                marginl: None,
                marginr: None,
                marginv: None,
                effect: None,
                text: None,
                color: None,
            }
        }
    }
}

impl Default for Dialogue {
    fn default() -> Dialogue {
        Dialogue {
            event: EventFormat::default()
        }
    }
}

impl Dialogue {
    fn to_string(&self) -> String {
        let mut dialogue_string = String::new();
        dialogue_string.push_str(EVENT_HEAD);
        dialogue_string.push_str(&(self.event.layer.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.start.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.end.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.style.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.name.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.marginl.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.marginr.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.marginv.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.effect.as_ref().unwrap_or(&"".to_owned()).to_owned() + ","));
        dialogue_string.push_str(&(self.event.text.as_ref().unwrap_or(&"".to_owned()).to_owned() + "\n"));

        return dialogue_string;
    }
}

impl Dialogue {
    /// set the layer
    /// Layer (any integer)
    /// Subtitles having different layer number will be ignored during the collusion detection.
    /// Higher numbered layers will be drawn over the lower numbered.
    pub fn set_layer(mut self, value: &str) -> Self {
		self.event.layer = Some(value.to_string());
		self
	}
    /// set the start time of the subtitle.
    /// Start Time of the Event, in 0:00:00:00 format ie. Hrs:Mins:Secs:hundredths. This is the time elapsed during script playback at which the text will appear onscreen. Note that there is a single digit for the hours!
    pub fn set_start(mut self, value: &str) -> Self {
		self.event.start = Some(value.to_string());
		self
    }
	/// set the end time of the subtitle.
    ///  End Time of the Event, in 0:00:00:00 format ie. Hrs:Mins:Secs:hundredths. This is the time elapsed during script playback at which the text will disappear offscreen. Note that there is a single digit for the hours!
    pub fn set_end(mut self, value: &str) -> Self {
		self.event.end = Some(value.to_string());
		self
	}
    /// set the style.
    /// Style name. If it is "Default", then your own *Default style will be subtituted.
    ///However, the Default style used by the script author IS stored in the script even though SSA ignores it - so if you want to use it, the information is there - you could even change the Name in the Style definition line, so that it will appear in the list of "script" styles.
    pub fn set_style(mut self, value: &str) -> Self {
		self.event.style = Some(value.to_string());
		self
	}
    /// set name.
    ///  Character name. This is the name of the character who speaks the dialogue. It is for information only, to make the script is easier to follow when editing/timing.
    pub fn set_name(mut self, value: &str) -> Self {
		self.event.name = Some(value.to_string());
		self
	}
    /// set the marginl
    /// 4-figure Left Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn set_marginl(mut self, value: &str) -> Self {
		self.event.marginl = Some(value.to_string());
		self
	}
    /// set the marginr
    ///  4-figure Right Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn set_marginr(mut self, value: &str) -> Self {
		self.event.marginr = Some(value.to_string());
		self
	}
    /// set the marginv
    ///  4-figure Bottom Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn set_marginv(mut self, value: &str) -> Self {
		self.event.marginv = Some(value.to_string());
		self
	}
    /// set effects for the Dialogue object.
    /// Transition Effect. This is either empty, or contains information for one of the three transition effects implemented in SSA v4.x
    /// The effect names are case sensitive and must appear exactly as shown. The effect names do not have quote marks around them.
    /// "Karaoke" means that the text will be successively highlighted one word at a time.
    /// Karaoke as an effect type is obsolete.
    pub fn set_effect(mut self, value: &str) -> Self {
		self.event.effect = Some(value.to_string());
		self
	}
    /// set the text for the subtitle.
    /// Subtitle Text. This is the actual text which will be displayed as a subtitle onscreen. Everything after the 9th comma is treated as the subtitle text, so it can include commas.
    /// The text can include \n codes which is a line break, and can include Style Override control codes, which appear between braces { }.
    pub fn set_text(mut self, value: &str) -> Self {
		self.event.text = Some(value.to_string());
		self
	}

    /// set the color of the subtitle.
    pub fn set_colour(mut self, color: HexColor) -> Self {
        let colour = AssFileOptions::get_ass_color_text(color);
        match &self.event.text {
            Some(text) => {
                let new_text = &(colour.clone() + text);
                // If you know a better way, please do create a pull request
                self = self.clone().set_text(new_text);
                self.event.color = Some(colour);
                self
            },
            None => {
                self.event.color = Some(colour.clone());
                self.set_text(&colour)
            }
        }
    }
}


impl Dialogue {
    /// get the layer of the subtitle
    pub fn get_layer(&self) -> Option<String> {
				return self.event.layer.clone();
	}
    /// get the start time of the `Dialogue`
    /// Start Time of the Event, in 0:00:00:00 format ie. Hrs:Mins:Secs:hundredths. This is the time elapsed during script playback at which the text will appear onscreen. Note that there is a single digit for the hours!
    pub fn get_start(&self) -> Option<String> {
				return self.event.start.clone();
    }
	/// get the end time of the `Dialogue`.
    ///  End Time of the Event, in 0:00:00:00 format ie. Hrs:Mins:Secs:hundredths. This is the time elapsed during script playback at which the text will disappear offscreen. Note that there is a single digit for the hours!
    pub fn get_end(&self) -> Option<String> {
				return self.event.end.clone();
	}
    /// get the style of the `Dialogue`.
    pub fn get_style(&self) -> Option<String> {
				return self.event.style.clone();
	}
    /// get the name of the `Dialogue`.
    ///  Character name. This is the name of the character who speaks the dialogue. It is for information only, to make the script is easier to follow when editing/timing.
    pub fn get_name(&self) -> Option<String> {
				return self.event.name.clone();
	}
    /// get the marginl of the `Dialogue`.
    /// 4-figure Left Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn get_marginl(&self) -> Option<String> {
				return self.event.marginl.clone();
	}
    /// get the marginr of the `Dialogue`.
    ///  4-figure Right Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn get_marginr(&self) -> Option<String> {
				return self.event.marginr.clone();
	}
    /// get the marginv of the `Dialogue`.
    ///  4-figure Bottom Margin override. The values are in pixels. All zeroes means the default margins defined by the style are used.
    pub fn get_marginv(&self) -> Option<String> {
				return self.event.marginv.clone();
	}
    /// get the effects for the Dialogue object.
    /// Transition Effect. This is either empty, or contains information for one of the three transition effects implemented in SSA v4.x
    /// The effect names are case sensitive and must appear exactly as shown. The effect names do not have quote marks around them.
    /// "Karaoke" means that the text will be successively highlighted one word at a time.
    /// Karaoke as an effect type is obsolete.
    pub fn get_effect(&self) -> Option<String> {
				return self.event.effect.clone();
	}
    /// get the text for the subtitle.
    /// Subtitle Text. This is the actual text which will be displayed as a subtitle onscreen. Everything after the 9th comma is treated as the subtitle text, so it can include commas.
    /// The text can include \n codes which is a line break, and can include Style Override control codes, which appear between braces { }.
    pub fn get_text(&self) -> Option<String> {
				return self.event.text.clone();
	}

    /// get the color of the subtitle.
    pub fn get_colour(&self) -> Option<String> {
        return self.event.color.clone()
    }
}

pub struct AssFileOptions{}

/// `script`, `v4` and `event` are fields in `Components`
#[derive(Clone, PartialEq, Debug)]
pub struct Components {
    /// instance holding the scirpt field.
    pub script: ScriptInfo,
    /// instance holding the V4 field of.
    pub v4: V4Format,
    /// instance holding the Events field of.
    pub events: Events,
}


pub struct Srt {
    srt_data: SrtData,
}

impl Srt {
    pub fn iter(&self) -> std::slice::Iter<'_, parser::SrtData> {
        let iterator = self.srt_data.iter();
        return iterator;
    }
}


/// # AssFile represents an instance of an existing `.ass` file.
///  The `AssFile::from_file function can be used to construct an `AssFile` from an existing `.ass
///  file`.

#[derive(Clone, PartialEq,Debug)]
pub struct AssFile{
    _ass_file: String,
    /// Each components present in a `.ass` file. 
    /// They are `script` `v4` and `events`.
    pub components: Components,
}

impl Deref for AssFile {
    type Target = Components;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl AssFile {
    pub fn new() -> AssFile {
        AssFile {
            _ass_file: String::new(),
            components: Components {
                script: ScriptInfo::new(),
                v4: V4Format::new(),
                events: Events::new(),
            }
        }
    }

    /// Load Subtitles from a SubRip file.
    ///
    /// # Example
    /// ``` rust
    /// let srt_file = AssFile::from_srt("sample.srt");
    ///
    /// for srt_seg in srt_file.iter() {
    ///    let start = &srt_seg.start;
    ///    let end = &srt_seg.end;
    ///    let text = &srt_seg.text;
    ///
    ///    println!("Start: {}\nEnd: {}\ntext: {}", start, end, text);
    /// 
    ///} ```
    pub fn from_srt(filename: &str) -> Srt {
        let file_contents = get_contents(filename).unwrap();
        let srtdata = parser::SrtData::new();
        let srt = srtdata.parse_srt(file_contents);

        Srt {
            srt_data: srt,
        }
    }
}

struct Parser; 
impl Parser {
    fn new() -> Parser {
        Parser
    }

    fn stringify_script(&self, scriptinfo: Vec<[&str; 2]>) -> String {
        let mut contents = String::new();

        for pair in scriptinfo {
            contents.push_str(&(pair[0].to_owned() + pair[1] + "\n"))
        }
        contents
    }

    fn combine_components(&self, components: &Components) -> String {
        let components = components.clone();
        let script = components.script;
        let v4 = components.v4;
        let events = components.events;
        let scriptinfo  = script.get_key_values();

        let script_data = &self.stringify_script(scriptinfo);
        let v4_data = &self.plug_v4(v4);
        let event_data = &self.plug_events(events);
        let total_data = format!("{}\n\n{}\n\n{}", script_data, v4_data, event_data);

        return total_data;
    }

    fn _plug_script(&self, script_lines: Vec<String>, scriptinfo: ScriptInfo) -> String {
        let mut new_lines = script_lines.clone();
        let mut total_lines = String::new();
        let script_type = scriptinfo.scripttype.unwrap();
        let playresx = scriptinfo.playresx.unwrap();
        let playresy = scriptinfo.playresy.unwrap();
        let scaledborderandshadow = scriptinfo.scaledborderandshadow.unwrap();
        let ycbcr_matrix = scriptinfo.ycbcr_matrix.unwrap();


        for (i, line) in script_lines.iter().enumerate() {
            if line.starts_with(SCRIPT_TYPE) {
                new_lines[i] = line[..SCRIPT_TYPE.len()].to_owned() + &script_type + "\n";
                continue
            } else if line.starts_with(SCRIPT_PLAYRESX){
                new_lines[i] = line[..SCRIPT_PLAYRESX.len()].to_owned() + &playresx + "\n";
                continue
            } else if line.starts_with(SCRIPT_PLAYRESY){
                new_lines[i] = line[..SCRIPT_PLAYRESY.len()].to_owned() + &playresy + "\n";
                continue;
            } else if line.starts_with(SCRIPT_SCALEDBORDERANDSHADOW){
                new_lines[i] = line[..SCRIPT_SCALEDBORDERANDSHADOW.len()].to_owned() + &scaledborderandshadow + "\n";
                continue;
            } else if line.starts_with(SCRIPT_YCBCR_MATRIX){
                new_lines[i] = line[..SCRIPT_YCBCR_MATRIX.len()].to_owned() + &ycbcr_matrix + "\n";
                continue;
            }
        }

        for line in new_lines {
            total_lines.push_str(line.as_str());
        }

        return total_lines;
    }

    fn plug_v4(&self, v4_info: V4Format) -> String {
        let array = v4_info.get_array();
        let mut values = Vec::new();
        let mut v4_lines = Vec::new();
        let mut total_v4 = String::new();
        v4_lines.push("[V4+ Styles]\n".to_string());
        v4_lines.push("Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n".to_string());
        v4_lines.push(V4_STYLE_HEAD.to_string());

        for (i, value) in array.into_iter().enumerate() {
            let style_val = value.clone().unwrap();
            if i < (array.len()-1) {
                values.push(style_val + ",");
            } else {
                values.push(style_val);
            }
        }

        values.push("\n".to_string());

        v4_lines.append(&mut values); 

        for line in v4_lines {
            total_v4.push_str(line.as_str());
        }
        return total_v4;
    }

    fn plug_events(&self, event_info: Events) -> String {
        let mut lines = Vec::new();
        let mut total_events = String::new();
        let dialogues = event_info.dialogues.dialogues;
        lines.push(EVENTS_HEADER.to_string() + "\n");
        lines.push("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text".to_string() + "\n");

        for dialogue in dialogues {
            let dialogue_line = dialogue.to_string();
            lines.push(dialogue_line);
        }
        for line in lines {
            total_events.push_str(line.as_str());
        }
        
        return total_events;
    }

    fn get_each_components(&self, file_contents: String) -> Components {
        let lines:Vec<&str> = file_contents.split("\n").collect();
        let script_lines = &self.get_info(&lines, SCRIPT_HEADER);
        let v4_lines = &self.get_info(&lines, V4_HEADER);
        let events_lines = &self.get_info(&lines, EVENTS_HEADER);

        let script = self.parse_script(script_lines.to_vec()).unwrap();
        let v4 = self.parse_v4(v4_lines.to_vec()).unwrap();
        let events = self.parse_event(events_lines.to_vec()).unwrap();

        Components {
            script,
            v4,
            events,
        }.clone()
    }
    fn parse_script(&self, script_lines: Vec<String>) -> Option<ScriptInfo> {
        let mut script_type: Option<String>= None;
        let mut script_playerresx: Option<String>= None;
        let mut script_playerresy: Option<String>= None;
        let mut script_scaledborderandshadow: Option<String>= None;
        let mut script_ycbcr_matrix: Option<String>= None;

        for line in &script_lines {
            if line.starts_with(SCRIPT_TYPE) {
                script_type = Some(line[SCRIPT_TYPE.len()..].to_owned());
                continue
            } else if line.starts_with(SCRIPT_PLAYRESX){
                script_playerresx= Some(line[SCRIPT_PLAYRESX.len()..].to_owned());
                continue
            } else if line.starts_with(SCRIPT_PLAYRESY){
                script_playerresy= Some(line[SCRIPT_PLAYRESY.len()..].to_owned());
                continue;
            } else if line.starts_with(SCRIPT_SCALEDBORDERANDSHADOW){
                script_scaledborderandshadow = Some(line[SCRIPT_SCALEDBORDERANDSHADOW.len()..].to_owned());
                continue;
            } else if line.starts_with(SCRIPT_YCBCR_MATRIX){
                script_ycbcr_matrix = Some(line[SCRIPT_YCBCR_MATRIX.len()..].to_owned());
                continue;
            }
        }
        println!("{:?}, {:?}, {:?}, {:?} {:?}", script_type, 
                 script_playerresx,
                 script_playerresy,
                 script_scaledborderandshadow,
                 script_ycbcr_matrix);

        let mut scriptinfo = ScriptInfo::new();
        let script_info = scriptinfo.
            set_scripttype(&script_type.unwrap()).
            set_playresx(&script_playerresx.unwrap()).
            set_playresy(&script_playerresy.unwrap()).
            set_scaledborderandshadow(&script_scaledborderandshadow.unwrap()).
            set_ycbcr_matrix(&script_ycbcr_matrix.unwrap()).clone();

        Some(script_info)
}
    fn parse_event(&self, event_lines: Vec<String>) -> Option<Events>{
        // let mut events = Vec::new();
        let mut raw_dialogues = Vec::new();
        let mut dialogues = Vec::new();
        
        for line in event_lines {
            if line.starts_with(EVENT_HEAD) {
                raw_dialogues.push(line[EVENT_HEAD.len()..].to_string());
            }
        }
        for dialogue in &raw_dialogues {
            let splitted_dialogue: Vec<&str> = dialogue.split(',').collect();
            let dialogue = Dialogue::new().
                set_layer(splitted_dialogue[0]).
                set_start(splitted_dialogue[1]).
                set_end(splitted_dialogue[2]).
                set_style(splitted_dialogue[3]).
                set_name(splitted_dialogue[4]).
                set_marginl(splitted_dialogue[5]).
                set_marginr(splitted_dialogue[6]).
                set_marginv(splitted_dialogue[7]).
                set_effect(splitted_dialogue[8]).
                set_text(splitted_dialogue[9]);
            
            dialogues.push(dialogue);
        }

        let dialogues = Dialogues {
            dialogues,
        };

        return Some(Events {
            dialogues,
        })


    }
    fn parse_v4(&self, v4_lines: Vec<String>) -> Option<V4Format>{
        let mut style_line: Option::<String> = None;
        for line in &v4_lines {
            if line.starts_with(V4_STYLE_HEAD) {
                style_line = Some(line[V4_STYLE_HEAD.len()..].to_string());
                break;
            }
        }
        if let Some(style_data) = style_line {
            let values: Vec<&str> = style_data.split(',').collect();
            println!("{:?}", values);

            let v4format = V4Format::new().
                set_name(values[0]).
                set_fontname(values[1]).
                set_fontsize(values[2]).
                set_primarycolour(values[3]).
                set_secondarycolour(values[4]).
                set_outlinecolour(values[5]).
                set_backcolour(values[6]).
                set_bold(values[7]).
                set_italic(values[8]).
                set_underline(values[9]).
                set_strikeout(values[10]).
                set_scalex(values[11]).
                set_scaley(values[12]).
                set_spacing(values[13]).
                set_angle(values[14]).
                set_borderstyle(values[15]).
                set_outline(values[16]).
                set_shadow(values[17]).
                set_alignment(values[18]).
                set_marginl(values[19]).
                set_marginr(values[20]).
                set_marginv(values[22]).
                set_encoding(values[22]).clone();

            return Some(v4format);
        } else {
            eprintln!("Unable to parse v4!");
            println!("{:?}", &v4_lines);
            return None
        }
//["Default", "Arial", "16", "&Hffffff", "&Hffffff", "&H0", "&H0", "0", "0", "0", "0", "100", "100", "0", "0", "1", "1", "0", "2", "10", "10", "10", "1"]
    }
    fn get_info(&self, lines: &Vec<&str>, header: &str) -> Vec<String> {
        let mut script_lines = Vec::new();
        let mut found_script_header = false;
        for line in lines {
            let line = if line.ends_with('\n') {
                &line[..line.len()-1]
            } else if line.ends_with('\r'){
                &line[..line.len()-1]
            }else if line.ends_with("\r\n"){
                &line[..line.len()-2]
            }else {
                line
            };
            if line == header{
                found_script_header = true;
                script_lines.push(line.to_string());
                script_lines.push("\n".to_string());
                continue
            }
            if found_script_header {
                if line.starts_with('[') {
                    break;
                } else if line.starts_with(';') {
                    continue;
                } else {
                    script_lines.push(line.to_string());
                }
            } else {
                continue;
            }
        }
        return script_lines;
    }
}

impl AssFile {
    /// Construct `AssFile` from an existing `.ass` file. 
    ///
    /// # Example
    /// ```rust
    /// # use ass_parser::AssFile;
    /// let mut ass_file = ass_parser::AssFile::from_file("src/subtitles.ass".to_string()).expect("error while reading file.");
    /// ```
    pub fn from_file(filename: &str) -> std::result::Result<AssFile, std::io::Error> {
        let file_contents = get_contents(&filename);
        let parser = Parser::new();
        match file_contents {
            Ok(contents) => {
                let components = parser.get_each_components(contents);
                Ok(
                    Self{
                    _ass_file: filename.to_string(),
                    components,
                })
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

}

impl AssFile {
    /// save an instance of `AssFile` to an `.ass` file. 
    /// # Example 
    /// ```rust
    /// use hex_color::HexColor;
    /// use ass_parser;
    /// use ass_parser::{AssFile, V4Format, AssFileOptions};

    ///
    /// fn main() -> Result<(), std::io::Error>{
    ///    let mut ass_file = ass_parser::AssFile::from_file("subtitles.ass".to_string())?;
    ///    ass_file.components.script 
    ///        .set_scripttype("v4.00+".to_string())
    ///        .set_playresx("384".to_string())
    ///        .set_playresy("288".to_string())
    ///        .set_scaledborderandshadow("yes".to_string())
    ///        .set_ycbcr_matrix("None".to_string());
    ///
    ///    ass_file.components.v4.set_v4(V4Format::default());
    ///
    ///    AssFile::save_file(&ass_file, "modified_subtitles.ass");
    /// }
    /// ```
    pub fn save_file(file_components: &AssFile, filename: &str) {
        let parser = Parser::new();
        let components = &file_components.components;

        let file_data = parser.combine_components(components);
        write_contents(filename, &file_data);
    }
}

impl AssFileOptions {
}

impl AssFileOptions{
    /// Get BB:GG:RR representation of colors in Hexadecimal form
    pub fn get_ass_color(color: HexColor) -> String {
        let red = color.r;
        let green = color.g;
        let blue = color.b;

        let red_hex = format!("{:x}", red);
        let green_hex = format!("{:x}", green);
        let blue_hex = format!("{:x}", blue);

        let reversed_hex_color = format!("{}{}{}", blue_hex, green_hex, red_hex);

        // let mut ass_format_color = format!(r"\c&H{}&", reversed_hex_color);
        let ass_format_color = format!("&H{}", reversed_hex_color);
        // ass_format_color.push('}');
        // ass_format_color = "{".to_owned() + &ass_format_color;

        return ass_format_color;
    }


    pub fn get_ass_color_text(color: HexColor) -> String {
        let red = color.r;
        let green = color.g;
        let blue = color.b;

        let red_hex = format!("{:x}", red);
        let green_hex = format!("{:x}", green);
        let blue_hex = format!("{:x}", blue);

        let reversed_hex_color = format!("{}{}{}", blue_hex, green_hex, red_hex);

        let mut ass_format_color = format!(r"\c&H{}&", reversed_hex_color);
        ass_format_color.push('}');
        ass_format_color = "{".to_owned() + &ass_format_color;

        return ass_format_color;
    }

    fn _change_ass_subtitle_color(ass_file: &str, color: HexColor) -> std::result::Result<(), std::io::Error>{
        if !check_path_exists(ass_file){
            eprintln!("ERROR: File {} does not exist", ass_file);
            return Ok(());
        }

        let mut file_data = String::new();
        let mut file_buffer = fs::File::open(ass_file)?;
        let ass_color = Self::get_ass_color(color);
        file_buffer.read_to_string(&mut file_data)?;

        let lines:Vec<&str> = file_data.split("\r\n").collect();
        let mut subtitle_lines = Vec::new();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.starts_with("Dialogue:") {
                subtitle_lines.push(line);
            }
        }

       for (_idx, line) in subtitle_lines.into_iter().enumerate() {
           let new_line = match line.rfind(",,") {
               Some(i) => {
                   let mut new_line = String::new();
                   new_line.push_str(&line[..i+2]);
                   new_line.push_str(&ass_color);
                   new_line.push_str(&line[i+2..]);
                   new_line.push_str("\r\n");
                   new_line
               },
               None => {
                   eprintln!("Unable to find match in line: {}", line);
                   line.to_string()
               }
           };
           new_lines.push(new_line);
       } 
       for line in &new_lines{
           println!("{}", line);
       }

       _write_dialogues(ass_file, new_lines);

        Ok(())
    }

}


//{\c&He3cb44&}

fn check_path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn _write_dialogues(filename: &str, dialogues: Vec<String>) {
    if !check_path_exists(filename){
        eprintln!("ERROR: File {} does not exist", filename);
        return
    }
    let mut file = fs::OpenOptions::new().read(true).write(true).open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let dialogue_idx = contents.find("Dialogue: ").unwrap();

    file.seek(std::io::SeekFrom::Start(dialogue_idx.try_into().unwrap())).unwrap();

    for line in dialogues {
        file.write(line.as_bytes()).unwrap();
    } 
}

fn write_contents(filename: &str, contents: &str) {
    let mut file = fs::File::create(filename).unwrap();
    file.write(contents.as_bytes()).unwrap();
}

fn get_contents(filename: &str) -> std::result::Result<String, std::io::Error>{
    if !check_path_exists(filename){
        return Err(std::io::ErrorKind::NotFound.into());
    }
    return fs::read_to_string(filename);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_contents() {
        use parser::SrtData;
        let file_contents = get_contents("examples/rapgod.srt").unwrap();

        let srt_data = SrtData::new();
        let srt_content = srt_data.parse_srt(file_contents);

        let test_srt_content = SrtData { 
            index: "0".to_string(),
			start: "0:00:01.50".to_string(),
			end: "0:00:04.90".to_string(),
			text: "Look, I was gonna go easy on you and not to hurt your feelings ".to_string(),
         };

        assert_eq!(test_srt_content, srt_content[0]);
    }

    #[test]
    fn test_from_file_wrong() {
        let result = AssFile::from_file("asdfasdf").map_err(|e| e.kind());
        let expected = std::result::Result::Err(std::io::ErrorKind::NotFound);

        assert_eq!(expected, result);
    }
}
