# A Pragmatic Introduction To Dagger on Android
If you're like me starting out with Dagger, you may have had thoughts such as:
**"What is Dagger?"**,
**"Why would I ever use this?"**,
**"Is it actually this hard to set up?"**

These are valid questions and some that I have quite a bit of experience with now up this point.

In order to answer the first question, it would be helpful if we learned what dependency injection is first.

## What is Dependency Injection?
To quote James Shore on his blog, *"The Art of Agile"*, "**“Dependency Injection” is a 25-dollar term for a 5-cent concept. 
"Dependency Injection" sounds scary, but it refers to an extremely simple concept we're all familiar with: taking parameters.**"

**No really**, it's just taking parameters. Taking parameters in functions, taking parameters in constructors, mutable members(on a class).

```kotlin
// Dependency Injection with a function
fun add(a: Int, b: Int): Int {
    return a + b
}

// NetworkHandler.kt
// Constructor Injection
class NetworkHandler(val client: OkHttpClient)

fun doSomething() {
    val client = OkHttpClient()
    val networkHandler = NetworkHandler(client) // Here we "inject" an OkHttpClient by passing a parameter to the constructor
}

// Member Injection
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

These are all examples of Dependency Injection and 99% of how it'll be performed with Dagger from a user-perspective.

## Okay, but, *why would I use this pattern?* What are we getting out of it?
Dependency Injection has quite a few benefits when applied throughout a codebase, but testing will probably be the main
one you see.

### Testing
Using DI consistently helps us write testable code. Suppose you have the following class and interface:

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
        val petListPresenter = PetListPresenter(petDataSource = sqlitePetDataSource) // Using constructor injection to satisfy the PetDataSource parameter in production code
    }

    fun Context.getDatabase(): PetDb {
        ...
        return db
    }
}
```

Still, nothing special is happening yet. All we did was abstract away *where* the data is coming from. We could have just as easily
used a `NetworkPetDataSource.kt` to retrieve our pets. However, when it comes time to test `PetListPresenter`, this pattern has a 
clear benefit: we can pass fakes/mocks into our object under test quickly, especially if we leverage Kotlin's `object` keyword:

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
        
        petListPresenter = PetListPresenter(fakePetDataSource) // Using constructor injection to satisfy the PetDataSource parameter in test code
    }

    @Test
    fun testGetPetById() {
        val pet = petListPresenter.getPetById(0)
        assertTrue(pet != null)
        assertEquals("Wheatley", pet.name)
    }
}
```

Since our `PetListPresenter` has no idea where the data is coming from(or knowledge of the implementation), we were able to spin up a `PetDataSource` on the fly
for our test. We have now used the DI pattern to make our testing significantly easier. Consider the case where `PetListDataSource` had defined `SQLitePetDataSource`
as a constructor parameter:

```kotlin
// PetListPresenter.kt
class PetListPresenter(val petDataSource: SQLitePetDataSource)
```

Then, our test would have had to do a bit more configuration to satisfy the constructor parameter for `SQLitePetDataSource` 
which might involve configuring an SQLite database on the file system then making sure we delete it when the tests are done 
so we don't waste computer resources. Yuck.

## I see. Dependency Injection is just a software pattern. But how come people use Dagger?
Dagger is a DI *framework*. It helps you facilitate this pattern in your code by building a graph of your dependencies, calling the constructors *for you*,
and getting your objects exactly where you need them. In the above example, if one had used Dagger, our `PetActivity` would have looked like this:

```kotlin
class PetActivity: AppCompatActivity()
{
    @Inject lateinit var petListPresenter: PetListPresenter // Public mutable member that gets injected later 
    
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
Take note of the **INJECTION** comment and recall members injection from earlier. That `Injector#inject(activity: PetActivity)` is
doing something like this under the hood:
```kotlin
// Injector.kt, generated by Dagger
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

Dagger takes care of this entire chain for you so your `PetActivity` doesn't have to care about `PetDb` and 
`SQLitePetDataSource`, it can focus entirely on `PetListPresenter`. 

Satisfying dependency chains so you can focus on only the object you need is what DI frameworks are great at.

## I see. Dagger is a DI Framework. But do we have to call `getInjector().inject(this)` all the time?
Unfortunately, we do. Lets take a look at the history of DI frameworks in Java to understand why.

Once upon a time, Dependency Injection frameworks like [Guice](1) used reflection to achieve their DI. At the time, these DI frameworks
weren't built with mobile devices in mind, they were built for servers. Reflection has a runtime cost in performance, but it's
an appropriate tradeoff for a server environment. When mobile devices came along, this was no longer the case and led to
app slowdowns. We needed a DI framework that was blazing fast for mobile devices but used less reflection.

[Dagger 1](1), originally created by Square(Block now), used a mixture of code generation and reflection to achieve DI. At 
compile-time, the injecting code was generated by using annotations (@Inject) in your own code that the Dagger compiler would look at
and process. It worked, but the reflection that was hanging around left a desire for more. What if we could make a DI framework 
that completely ignored reflection and just used code generation at compile-time? This would also yield a major advantage for
Dagger 2: failing early. When a dependency couldn't be satisfied(it wasn't on the DI frameworks graph), developers would
find out at runtime what had happened instead of compile-time. Early failures lead to faster fixes.

Thus, Dagger 2 was born. Dagger 2 *ignored reflection entirely*. Instead of making your users pay a performance cost, our Gradle 
builds would eat the cost in build time and generate injecting code at compile time. It was now possible to use a Dependency Injection 
framework without reflection.

This made DI with the assistance of a framework *blazing fast* and appropriate for mobile. We could have clean code without
having to call a ton of constructors ourselves. But *we have to hook into that generated code somewhere.*

So, yes, we do have to call `getInjector().inject(this)` because this is how we hook into Dagger's
generated code. I'm skipping a lot for the sake of brevity but this is generally how injecting Fragments and Activities
on Android goes. Since we can't have custom constructors for Fragments and Activities(unless you're using minSdk 28) either,
this means we have to use Members Injection(not preferred).

## Reflection Vs Code Generation
It isn't all sunshine and rainbows. One of the most obvious tradeoffs for code generation vs reflection was we have to call 
the generated code somewhere instead of it being automatic, like Guice.

We also cannot inject private members using code generation, but reflection can. In some ways, Dagger 2 can be harder to use
than a reflection-based DI framework. However, these sacrifices afford us an extremely fast dependency injector that's appropriate for Android. 

In the next article, we'll discuss how to configure Dagger 2 in an Android app.

[1]: https://github.com/google/guice
[2]: https://github.com/square/dagger
