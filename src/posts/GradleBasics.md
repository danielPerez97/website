# Using Licensee to Automate Your Open-Source Licenses on Android Using Jetpack Compose

On June 8, 2021, Jake Wharton from Square's Cash App team posted [this blog post][1] detailing a new tool they'd been using
internally called Licensee. Licensee examines your applications dependencies with a gradle plugin and produces a JSON
file that can be parsed into an app. It's also completely agnostic to whether or not you are using an Android app or 
building a regular JVM application(Compose Desktop anyone?).

# Getting Licensee
Per [Licensee's documentation][2], you'll need to add the gradle plugin to your classpath and apply the plugin to
whichever module whose dependency graph you are planning to check(for Android, let's use the app module):

```kotlin
// Project build.gradle
buildscript {
  repository {
    mavenCental()
  }
  dependencies {
    classpath 'app.cash.licensee:licensee-gradle-plugin:1.2.0'
  }
}

// Put this in your App Module

// Method 1
apply plugin: 'app.cash.licensee'

// Method 2
plugins {
    id("app.cash.licensee")
}
```

You're all set to use Licensee. Let's take a look at configuring it to only allow certain licenses.

# Configuring Licenses

# Creating artifacts.json

# Copying artifacts.json to your /res folder

# Using Moshi to Parse JSON files

# Rendering your list in Jetpack Compose 

[1]: https://code.cash.app/gradle-dependency-license-validation
[2]: https://github.com/cashapp/licensee#usage
