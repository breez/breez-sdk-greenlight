plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android") version "1.6.10"
    id("maven-publish")
}

repositories {
    mavenCentral()
    google()
}

android {
    compileSdk = 31

    defaultConfig {
        minSdk = 21
        targetSdk = 31
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
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
    implementation("net.java.dev.jna:jna:5.8.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.4.0")
    implementation("androidx.core:core-ktx:1.7.0")
}

val libraryVersion: String by project

publishing {
    repositories {
        maven {
            name = "breezGitHubPackages"
            url = uri("https://maven.pkg.github.com/breez/breez-sdk")
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
                        url.set("https://github.com/breez/breez-sdk/blob/main/LICENSE")
                    }
                }
                scm {
                    connection.set("scm:git:github.com/breez/breez-sdk-ffi.git")
                    developerConnection.set("scm:git:ssh://github.com/breez/breez-sdk.git")
                    url.set("https://github.com/breez/breez-sdk")
                }
            }
        }
    }
}
