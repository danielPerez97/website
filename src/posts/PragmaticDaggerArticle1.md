# A Pragmatic Introduction To Dagger on Android Part 1: Why use Dagger?

Dagger 2 is one of the most infamous tools in Android development and yet one of the most indispensable. Newcomers have
struggled to understand why it's used, what Dependency Injection is, and how to set Dependency Injection up in their own
projects.
If you are currently struggling with this topic, you may share in some of these thoughts I used to have:

***"What is Dagger?"***

***"Why would I ever use this?"***

***"Is it actually this hard to set up?"***

In this article, I'll attempt to answer all of these questions before showing you how to implement Dagger in your own
Android app in Part 2.

In order to answer **"What is Dagger?"**, it would be helpful if we learned what Dependency Injection is first.

## What is Dependency Injection?

James Shore [worded it perfectly][1] on his blog, *"The Art of Agile"*:

**“Dependency Injection” is a 25-dollar term for a 5-cent concept.
"Dependency Injection" sounds scary, but it refers to an extremely simple concept we're all familiar with: taking
parameters.**

**No really**, it's just taking parameters. Taking parameters in functions, taking parameters in constructors, and
setting mutable members (on a class).

In the next code sample, there are three demonstrations of different kinds of Dependency Injection:

* Constructor Injection
* Method Injection
* Field Injection

```kotlin
// Constructor Injection
class NetworkHandler(val client: OkHttpClient)

fun doSomething() {
    val client = OkHttpClient()
    val networkHandler = NetworkHandler(client) // Here we "inject" an OkHttpClient by passing a parameter to the constructor
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

These are all examples of Dependency Injection in object-oriented programming.

## Okay, but, *why would I use this pattern?* What are we getting out of it?

Dependency Injection has quite a few benefits when applied throughout a codebase, but testing is one of the most talked
about especially with discussion around Dagger.

***Note: From this point forward I will shorthand Dependency Injection with "DI".***

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

Still, nothing special is happening yet. All we did was abstract away *where* the data is coming from. We could have
just as easily used a `NetworkPetDataSource.kt` to retrieve our pets. However, when it comes time to
test `PetListPresenter`, this
pattern has a clear benefit: we can pass fakes/mocks into our object under test quickly, especially if we leverage
Kotlin's `object`
keyword.

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

Since our `PetListPresenter` has no idea where the data is coming from (or knowledge of the implementation), we were
able to spin up a `PetDataSource` on the fly for our test. We have now used the DI pattern to make our testing
significantly
easier. Consider the case where `PetListDataSource` had defined `SQLitePetDataSource` as a constructor parameter:

```kotlin
// PetListPresenter.kt
class PetListPresenter(val petDataSource: SQLitePetDataSource)
```

In this case, our test would have had to do a bit more configuration to satisfy the constructor parameter
for `SQLitePetDataSource`, which might involve setting up an SQLite database on the file system then making sure we
delete it when the tests are done so we don't waste computer resources. Yuck.

## I see. Dependency Injection is just a software pattern. But how come people use Dagger?

Dagger is a DI *framework*. It helps you facilitate this pattern in your code by building a graph of your dependencies,
calling the constructors *for you*,
and getting your objects exactly where you need them. In the above example, if one had used Dagger, our `PetActivity`
would have looked like this:

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

Take note of the **INJECTION** comment and recall members injection from earlier.
That `Injector#inject(activity: PetActivity)` is
doing something like this under the hood:

```kotlin
// Injector.kt. Dagger will generate similar code to this but in Java.
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
`SQLitePetDataSource`. It can focus entirely on `PetListPresenter`.

Satisfying dependency chains so you can focus on only the object you need is what DI frameworks are great for.

## Okay, so Dagger is a DI Framework. But do we have to call `getInjector().inject(this)` all the time?

Yes. It is imperative that we call Dagger's generated code *somewhere* for every object we'd like to retrieve from
the Dagger graph.* In the case of Members Injection, something called a `MembersInjector` gets generated for every
object whose members are annotated with `@Inject`. We must call this generated `MembersInjector` in order to inject
our objects.

In the case of Android Framework types like Activity, you cannot use constructor injection** because the Android OS will
be creating these objects, not Dagger. Hence, Members Injection becomes the next best option.

## Reflection Vs Code Generation

It isn't all sunshine and rainbows. One tradeoff for code generation vs reflection is that we have to call the generated
code somewhere instead of a more automatic-feeling solution, like Guice or Spring Dependency Injection.

We also cannot inject private members using code generation, but reflection can. In some ways, Dagger 2 can be harder to
use
than a reflection-based DI framework. However, these sacrifices afford us an extremely fast dependency injector that's
appropriate for Android.

## Wrapping up

Let's revisit those 3 questions from the beginning of the article and give an answer for each of them now:

***"What is Dagger?"***  
Dagger is a Dependency Injection Framework, and there are more like it including Guice and Spring Dependency Injection.

***"Why would I ever use this?"***  
To help you facilitate the Dependency Injection pattern in your code, which helps you test and decouple your code.

***"Is it actually this hard to set up?"***  
Yeah, I'd go ahead and say Dagger 2 is a bit hard to set up, but that's partly because it's based on 100% code
generation instead of reflection. We make this tradeoff for major performance gains on mobile devices.

## Part 2

Stay tuned for Part 2 where we'll discuss how to configure Dagger 2 in an Android app.

## Notes

#### "Yes. It is imperative that we call Dagger's generated code" *somewhere* for every object we'd like to retrieve from the Dagger graph."

* This isn't to say we can't be more clever about calling our `MembersInjector`'s. [Slack largely hid this][4] by using
  an extension to Dagger called [Anvil][5].

#### "In the case of Android Framework types like Activity, you cannot use constructor injection..."

* If you're using `minSdk=28`, you can use [`AppComponentFactory`][6] and [`FragmentFactory`][6] to use constructor
  injection on Activities, Fragments, Services, Content Providers, etc. This will require a feature we'll discuss in the
  future called Multibindings.

[1]: http://www.jamesshore.com/v2/blog/2006/dependency-injection-demystified

[2]: https://github.com/google/guice

[3]: https://github.com/square/dagger

[4]: https://slack.engineering/extending-anvil-for-fun-and-profit/

[5]: https://github.com/square/anvil

[6]: https://developer.android.com/reference/android/app/AppComponentFactory
