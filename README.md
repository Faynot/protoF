Twitch - faynot_ /
YT - Faynot
<div><img alt="Twitch Status" src="https://img.shields.io/twitch/status/faynot_">
<img alt="YouTube Channel Subscribers" src="https://img.shields.io/youtube/channel/subscribers/UCAi7bnW3bOA0epKrKUA8IXw"></div>
<div><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/Faynot/protoF">
<img alt="GitHub License" src="https://img.shields.io/github/license/Faynot/protoF"><img alt="GitHub Downloads (all assets, all releases)" src="https://img.shields.io/github/downloads/Faynot/protoF/total">
</div>


<h1>Wait, there will be documentation later...</h1>

<p>for now look at the cat</p>
<img src="./balls.png"></img>

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

<h1>How to use?</h1>
<p>1. Just download latest release</p>
<p>2. Move file in "protof.exe" to some directory</p>
<p>3. Place protof.exe in PATH</p>

<h1>How to build?</h1>
<pre><code>cargo build</pre></code>
<p>its all.</p>
