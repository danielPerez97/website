---
title: A Pragmatic Introduction To Metro on Android - Why use Metro?

tags:
  - Android
  - Metro
  - Dependency Injection

lead: I struggled to learn about Dependency Injection + A DI Framework
---

*Note: I adapted this article from one of my [previous articles][1]. Some of this article remains the exact same, some I
attempted to improve three years later. It is completely possible some of the edits may not resonate with you.*

[Metro][2] has become my favorite tool in Android Development. More than Compose, more than Navigation 3, more than
RxJava/Flows as well as many others that made Android Development better. All these libraries were necessary to make
Android Development more ergonomic and friendly, but none have made me happier to use than a good Dependency Injection
framework. There was a time I would have told you Dagger 2 was more indispensable to me than Compose. I'm pleased to
tell you this statement has a new king: Metro is more indispensable to me than Dagger 2.

I've written about why and how you should use a Dependency Injection framework like Dagger 2 in the past. This article
attempts to be an improvement over the previous one but using Metro as the dependency injection tool instead.

## The Junior Developer Struggle

When I started with Android, terminology very quickly became overwhelmed me. Terms like "Dependency Injection"
and "Instrumented Testing" scared the hell out of me. I wrote this article with my younger self in mind + any newer
Android Developers who may be starting out. Keep going, don't let these terms scare you, and learn from the good folks
in the Android community. Listening to and learning from the Android Community made me a better programmer, and I'm
certain it can apply to you too.

## Early Questions

Before getting into Metro itself, let's talk about Dependency Injection for a moment. In my experience, I've found it
much easier to learn about a technology when you know the problem it set out to solve first. I'll frame it through some
of the questions I had when I was initially learning about this topic*:

***"What is Metro?"***

***"Why would I ever use this?"***

***"Is it actually this hard to set up?"***

We'll come back to these, but we must talk about Dependency Injection first.

## What is Dependency Injection?

James Shore [worded it perfectly][3] on his blog, *"The Art of Agile"*:

**“Dependency Injection” is a 25-dollar term for a 5-cent concept.
"Dependency Injection" sounds scary, but it refers to an extremely simple concept we're all familiar with: taking
parameters.**

**No really**, it's just taking parameters, or, **injecting** them. Injecting parameters in constructors, injecting
parameters in functions, and injecting parameters into mutable members on a class. Listed out, we have:

* Constructor Injection
* Method Injection
* Members Injection

```kotlin
// Constructor Injection
class NetworkHandler(val client: OkHttpClient)

fun doSomething() {
    val client = OkHttpClient()
    
    // Here we "inject" an OkHttpClient by passing a parameter to the constructor
    val networkHandler = NetworkHandler(client) 
}

// Method Injection
fun setPet(pet: Pet) {
    this.pet = pet
}

// Members Injection
class MyActivity()
{
    lateinit var okHttpClient: OkHttpClient // <- Public, mutable member
}

class Injector(val appOkHttpClient: OkHttpClient)
{
    fun injectMyActivity(activity: MyActivity) {
        activity.okHttpClient = okHttpClient // <- Here we set the member, or "inject" it
    }
}
```

## Okay, but, *why would I use this pattern?* What are we getting out of it?

Dependency Injection has quite a few benefits when applied throughout a codebase, and testing is one of them (it's also
very easy to show).

### Testing

Using Dependency Injection consistently helps us write testable code. Suppose you have the following class and
interface:

```kotlin
// Pet.kt
data class Pet(val id: Long, val name: String)

// PetDataSource.kt
interface PetDataSource
{
    fun getPets(): List<Pet>
}
```

Nothing special yet, but here's how we might use it in a `PetListPresenter` class:

```kotlin
// PetListPresenter.kt
class PetListPresenter(val petDataSource: PetDataSource) // Constructor Injection
{
    fun getPetByid(id: Long): Pet? {
        return petDataSource.getPets().find { it.id == id }    
    }
}


// SQLitePetDataSource.kt
class SQLitePetDataSource(val petDb: PetDb): PetDataSource {
    override fun getPets(): List<Pet> {
        ...
        return pets
    }
}

// PetActivity.kt
...
class PetActivity: AppCompatActivity()
{
    override fun onCreate() {
        super.onCreate()
        val sqlitePetDataSource = SQLitePetDataSource(db = context.getDatabase())
        
        // Using constructor injection to satisfy the PetDataSource parameter in production code
        val petListPresenter = PetListPresenter(petDataSource = sqlitePetDataSource) 
    }

    fun Context.getDatabase(): PetDb {
        ...
        return db
    }
}
```

Here, we've hidden where the data comes from (an SQLite database) by hiding the implementation behind the
`PetDataSource` interface. We could have directly used a `NetworkPetDataSource.kt` to retrieve our pets. However, when
it comes time to test `PetListPresenter`, this pattern has a clear benefit: we can pass fakes/mocks into our object
under test quickly, especially if we leverage Kotlin's `object` keyword.

```kotlin
// TestPetListPresenter.kt

class TestPetListPresenter                                                      
{
    private lateinit var petListPresenter: PetListPresenter
    
    @Before
    fun setup() {
        val fakePetDataSource: PetDataSource = object: PetDataSource {
            override fun getPets(): List<Pet> {
                return listOf(Pet(id = 0, name = "Wheatley"))
            }   
        }
        
        // Using constructor injection to satisfy the PetDataSource parameter in test code
        petListPresenter = PetListPresenter(fakePetDataSource) 
    }

    @Test
    fun testGetPetById() {
        val pet = petListPresenter.getPetById(0)
        assertTrue(pet != null)
        assertEquals("Wheatley", pet.name)
    }
}
```

Since our `PetListPresenter` has no idea where the data is coming from (or knowledge of the implementation), we were
able to spin up a `PetDataSource` on the fly for our test. We have now used the DI pattern to make our testing
significantly easier. Consider the case where `PetListDataSource` had defined `SQLitePetDataSource` as a constructor
parameter:

```kotlin
// PetListPresenter.kt
class PetListPresenter(val petDataSource: SQLitePetDataSource)
```

In this case, our test would have had to do a bit more configuration to satisfy the constructor parameter for
`SQLitePetDataSource`, which might involve setting up an SQLite database on the file system then deleting said files
resources. Not ideal for a unit test.

## I see. Dependency Injection is just a software pattern. But how come people use Metro?

Metro is a DI *framework*. It helps you facilitate this pattern in your code by building a [directed acyclic graph][11] of
your dependencies, calling the constructors *for you*, and getting your objects exactly where you need them. In the
above example, if one had used Metro, our `PetActivity` would have looked like this:

```kotlin
class PetActivity: AppCompatActivity()
{
    // Public mutable member that gets injected later 
    @Inject lateinit var petListPresenter: PetListPresenter 
    
    override fun onCreate() {
        getInjector().inject(this) // INJECTION
        super.onCreate()
        // You can use petListPresenter anywhere you need to now, as long as onCreate() was called
    }
    
    fun getInjector(): Injector {
        ...
        return injector
    }
}
```

Take note of the **INJECTION** comment and recall members injection from earlier. That
`Injector#inject(activity: PetActivity)` is doing something like this under the hood:

```kotlin
// Injector.kt. Metro will generate similar code to this but in Java.
class Injector(...)
{
    ...
    fun inject(activity: PetActivity) {
        // Note: sqlitePetDataSource could be a member of this Injector class, or it might be provided by a factory
        activity.petListPresenter = PetListPresenter(sqlitePetDataSource) // <- Here we set the member, or "inject" it
    }
}
```

Let's pause for a moment and look at our chain of dependencies for `PetListPresenter`:

`PetListPresenter` -> `SQLitePetDataSource` -> `PetDb`

Metro takes care of this entire chain for you so your `PetActivity` doesn't have to care about `PetDb` and
`SQLitePetDataSource`. It can focus entirely on `PetListPresenter`.

Satisfying dependency chains **so you can focus on only the object you need** is what DI frameworks are great for.

## Okay, so Metro is a DI Framework. But do we have to call `getInjector().inject(this)` all the time?

Ideally you wouldn't have to. Your `minSdk` probably decides for you.

If you have at least `minSdk = 28`, you can prefer constructor injection. `minSdk = 28` [introduced API's][6]
like `AppComponentFactory` and `FragmentFactory` so you can get constructor injection on Activities, Fragments,
Services, Content Providers, etc. [MetroX Android][7] is quite good at hooking into these API's for you.

If you are below `minSdk = 28`, then it is imperative that we call Metro's generated code *somewhere* for every object
we'd like to retrieve from the Metro graph.* In the case of Members Injection, something called a `MembersInjector` gets
generated for every object whose members are annotated with `@Inject`. We must call this generated `MembersInjector` in
order to inject our objects.

This restriction is because of the Android Framework pre SDK 28: types like Activity cannot use constructor injection
because the Android OS will always be creating these objects, not Metro. Hence, Members Injection becomes the next best
option.

## Reflection Vs Code Generation

A reflection-based Dependency Injection framework like [Guice][10] can be more "magic" because you don't have to call
the generated code anywhere in order to get your objects. Guice was originally made for server applications though, not
mobile devices. In the early 2010's, Guice on Android wasn't unheard of, but the performance cost was too much.

A better-performing solution was needed, and a better-performing solution is what we got: [Dagger][8].

Dagger 1 did not use 100% code generation, but it was more appropriate for mobile than Guice. 100% code generation was
finally achieved with [Dagger 2][9].

For all the performance gains achieved with 100% code generation, there were still some tradeoffs. Injecting private
members isn't possible with code generation, but is possible with reflection. Guice can also feel easier to use where
Metro can feel harder. For these sacrifices though, we got an extremely fast dependency injector that's appropriate for
a resource-constrained mobile device.

## Wrapping up

Let's revisit those 3 questions from the beginning of the article and give an answer for each of them now:

***"What is Metro?"***  
Metro is a Dependency Injection Framework, and there are more like it including Guice and Spring Dependency Injection.

***"Why would I ever use this?"***  
To help you facilitate the Dependency Injection pattern in your code, which helps you test and decouple your code.

***"Is it actually this hard to set up?"***  
It can feel like a lot of ceremony to get it going, but it's ceremony you only have to do once, and then you can largely
forget about it. It helps to know it's built on code-generation instead of reflection so you know why you're writing the
ceremony you're writing. We make this tradeoff for major performance gains on mobile devices.

I think Metro is even easier than Dagger 2 to set up, and it's largely because of semantics. `@BindingContainer` is so
much more forward about its intent than Dagger's `@Module` annotation for example. Or the Metro docs telling you to
create an `AppGraph` interface, not an `AppComponent`.

#### * "Yes. It is imperative that we call Metro's generated code"

* This isn't to say we can't be more clever about calling our `MembersInjector`'s. [Slack largely hid this][4] by using
  an extension to Metro called [Anvil][5].

[1]: https://danperez.dev/pragmatic-dagger-part-1/

[2]: https://zacsweers.github.io/metro/

[3]: http://www.jamesshore.com/v2/blog/2006/dependency-injection-demystified

[4]: https://slack.engineering/extending-anvil-for-fun-and-profit/

[5]: https://github.com/square/anvil

[6]: https://developer.android.com/reference/android/app/AppComponentFactory

[7]: https://zacsweers.github.io/metro/latest/metrox-android/

[8]: https://square.github.io/dagger/

[9]: https://dagger.dev/

[10]: https://github.com/google/guice

[11]: https://en.wikipedia.org/wiki/Directed_acyclic_graph
