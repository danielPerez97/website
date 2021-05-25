### Gradle Basics
Gradle is a controversial tool, if /r/androiddev is to be trusted.
Still, every Android Developer has used it by now and I'm here to 
try and convince you to use Gradle modules.

### What is a module?
It's a library. That's it. No really, a module is a library that
resides within your Gradle project. Nobody else except you can 
throw your module/library into their `dependencies` block, but you 
can. This can be achieved similarly to what you're already used to,
just call the `project` function inside the `dependencies` function,
then specify your module as a string.

Kotlin:  
`implementation( project( "my-module-name" ) )`

Groovy:  
`implementation project 'my-module-name''`

Have you ever seen that mysterious settings.gradle(or settings.gradle.kts)
file? That is where you specify modules inside your project. In 
the above example, that is where we would specify "my-module-name" so that
the `project` function knows what it's looking for.

### Why should I use them?
Early in my Android development journey, I ran into a common problem:
I could not structure my code. I could write code, this was evidenced
way-too-big activity classes with multiple instances of duplicated code.

#### Dependencies
"Gradle is the tool I dump my libraries into." True

"Gradle seems to"


#### Structure
Goal: To Simplify Gradle by introducing modules
Audience: Someone who has used gradle, but hasn't used modules.
