
<div><a href="https://www.twitch.tv/faynot_" ><img alt="Twitch Status" src="https://img.shields.io/twitch/status/faynot_"></a>
<img src="./protof.png"></img>
<h1>Look</h1>

  <br />
<div><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/Faynot/protoF">
<img alt="GitHub License" src="https://img.shields.io/github/license/Faynot/protoF"><a href="https://github.com/Faynot/protoF/releases"><img alt="GitHub Downloads (all assets, all releases)" src="https://img.shields.io/github/downloads/Faynot/protoF/total"></a>
</div>


<h1>Documentation</h1>
<a href="https://glint.gitbook.io/protof" ><img src="https://glint.gitbook.io/~gitbook/image?url=https%3A%2F%2F3037555305-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F0SLxuIGpIhEnJCMb6vgv%252Fuploads%252F87NTB6oDuF3lTlm69S6J%252F123.png%3Falt%3Dmedia%26token%3D863a84d8-c754-4954-97c1-fd4f66919f1d&width=768&dpr=1&quality=100&sign=63632c0&sv=1"></img><a>


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

There are no releases on macos and linux yet, as it does not work well on unix-like operating systems
