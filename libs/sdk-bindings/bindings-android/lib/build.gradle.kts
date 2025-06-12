plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android") version "1.9.21"
    id("maven-publish")
    kotlin("plugin.serialization") version "1.9.21"
}

repositories {
    mavenCentral()
    google()
}

android {
    namespace = "technology.breez"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        consumerProguardFiles("consumer-rules.pro")
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    buildTypes {
        getByName("release") {
            @Suppress("UnstableApiUsage")
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }

    publishing {
        singleVariant("release") {
            withSourcesJar()
        }
    }
}

dependencies {
    implementation("com.squareup.okio:okio:3.6.0")
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.3")
    implementation("org.jetbrains.kotlinx:atomicfu:0.23.1")
}

val libraryVersion: String by project

publishing {
    repositories {
        maven {
            name = "breezReposilite"
            url = uri("https://mvn.breez.technology/releases")
            credentials(PasswordCredentials::class)
            authentication {
                create<BasicAuthentication>("basic")
            }
        }
        maven {
            name = "breezGitHubPackages"
            url = uri("https://maven.pkg.github.com/breez/breez-sdk-greenlight")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
    publications {
        create<MavenPublication>("maven") {
            groupId = "breez_sdk"
            artifactId = "bindings-android"
            version = libraryVersion

            afterEvaluate {
                from(components["release"])
            }

            pom {
                name.set("breez-sdk")
                description.set("The Breez SDK enables mobile developers to integrate Lightning and bitcoin payments into their apps with a very shallow learning curve.")
                url.set("https://breez.technology")
                licenses {
                    license {
                        name.set("MIT")
                        url.set("https://github.com/breez/breez-sdk-greenlight/blob/main/LICENSE")
                    }
                }
                scm {
                    connection.set("scm:git:github.com/breez/breez-sdk-greenlight.git")
                    developerConnection.set("scm:git:ssh://github.com/breez/breez-sdk-greenlight.git")
                    url.set("https://github.com/breez/breez-sdk-greenlight")
                }
            }
        }
    }
}
