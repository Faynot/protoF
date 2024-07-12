

<img src="./protof.png"></img>
<h1>Look</h1>
<div><a href="https://www.twitch.tv/faynot_" ><img alt="Twitch Status" src="https://img.shields.io/twitch/status/faynot_"></a>
  <br />
<a href="https://www.youtube.com/@Faynot__" ><img alt="YouTube Channel Subscribers" src="https://img.shields.io/youtube/channel/subscribers/UCAi7bnW3bOA0epKrKUA8IXw"></a><br /></div><br /><br />
<div><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/Faynot/protoF"><br />
<img alt="GitHub License" src="https://img.shields.io/github/license/Faynot/protoF"><a href="https://github.com/Faynot/protoF/releases"><img alt="GitHub Downloads (all assets, all releases)" src="https://img.shields.io/github/downloads/Faynot/protoF/total"></a>
</div>


<h1>Documentation</h1>

<a href="https://glint.gitbook.io/protof">
<img src="./123.png"></img>
</a>
<br />
 ⬆️ click this  ⬆️

<h1>hello! hi! hello!</h1>
<p>This is a universal console utility for developers! Here, at the time of version 0.2, you can create sh scripts, which in terms of functionality, in fact, can be called an interpreter, and there is also a command for formatting your code in a certain number of languages, using other utilities. I put everything in one pile, made it all convenient, and now you can use it in several commands</p>

<p>The list of supported languages at the moment:</p>

<ul>
  <li>
    Java
  </li>
    <li>
    Python
  </li>
    <li>
    Rust
  </li>
    <li>
    JavaScript
  </li>
    <li>
    TypeScript
  </li>
    <li>
    JSX
  </li>
    <li>
    Flow
  </li>
    <li>
    JSON
  </li>
    <li>
    HTML
  </li>
    <li>
    Vue
  </li>
    <li>
    Angular
  </li>
    <li>
    Ember / Handlebars
  </li>
    <li>
    GraphQL
  </li>
    <li>
    CSS
  </li>
    <li>
    Less
  </li>
    <li>
    Markdown
  </li>
    <li>
    YAML
  </li>
</ul>

<p>Yes, I have combined several language formatters into one utility, and if they are not installed, they will be installed by you (this should be the case, but there are some bugs that I cannot fix at the moment)</p>

<h1>An illustrative example of code formatting:</h1>
<pre><code>

// before
fn main() {
let number = 5;

    if number < 0 {
        println!("Nubmer is negative");
    } else if number == 0 {
        println!("Number is zero");
    } else if number > 0 && number < 10 {
        println!("Number is positive and less than 10");
    } else {
        println!("Number is positive and greater than or equal to 10");
    }
}
</pre></code>
<pre><code>
  // after
fn main() {
    let number = 5;

    match number {
        n if n < 0 => println!("Nubmer is negative"),
        0 => println!("Number is zero"),
        n if n > 0 && n < 10 => println!("Number is positive and less than 10"),
        _ => println!("Number is positive and greater than or equal to 10"),
    }
}

</pre></code>

<h1>Screenshots</h1>
<img src="https://media.discordapp.net/attachments/1261280181977616458/1261280209924001894/Screenshot_19.png?ex=6692623b&is=669110bb&hm=b078ef4d7f896a04c729b2d7dbdf65f1f65ee9ac4ed13207186b0ab95a9a429c&=&format=webp&quality=lossless&width=692&height=484" />
<img src="https://media.discordapp.net/attachments/1261280181977616458/1261280210171596810/Screenshot_20.png?ex=6692623b&is=669110bb&hm=dea9212948d95cb26c54c173b2b83534f2566466a1746d15f9c6ac8531fc53ae&=&format=webp&quality=lossless" />
<img src="https://media.discordapp.net/attachments/1261280181977616458/1261280210414862417/Screenshot_18.png?ex=6692623b&is=669110bb&hm=65b3e157e0f3db7fba72913a865d49190d0866443268ac07ba21aa6b199c83af&=&format=webp&quality=lossless" />


<h1>How to use?</h1>
<p>1. Just download latest release</p>
<p>2. Move file in "protof.exe" to some directory</p>
<p>3. Place protof.exe in PATH</p>

<h1>How to build?</h1>
<pre><code>cargo build</pre></code>
<p>its all.</p>

There are no releases on macos and linux yet, as it does not work well on unix-like operating systems
