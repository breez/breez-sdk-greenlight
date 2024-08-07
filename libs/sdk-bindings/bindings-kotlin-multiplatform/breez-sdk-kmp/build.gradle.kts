plugins {
    kotlin("multiplatform")
    id("com.android.library")
    id("maven-publish")
}

apply(plugin = "kotlinx-atomicfu")

kotlin {
    // Enable the default target hierarchy
    applyDefaultHierarchyTemplate()

    androidTarget {
        compilations.all {
            kotlinOptions {
                jvmTarget = JavaVersion.VERSION_17.majorVersion
            }
        }

        publishLibraryVariants("release")
    }

    jvm {
        compilations.all {
            kotlinOptions.jvmTarget = JavaVersion.VERSION_17.majorVersion
        }
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        val platform = when (it.targetName) {
            "iosSimulatorArm64" -> "ios_simulator_arm64"
            "iosArm64" -> "ios_arm64"
            "iosX64" -> "ios_x64"
            else -> error("Unsupported target $name")
        }

        it.compilations["main"].cinterops {
            create("breezCInterop") {
                defFile(project.file("src/nativeInterop/cinterop/breez.def"))
                includeDirs(project.file("src/nativeInterop/cinterop/headers/breez_sdk"), project.file("src/libs/$platform"))
            }
        }
    }

    sourceSets {
        all {
            languageSettings.apply {
                optIn("kotlinx.cinterop.ExperimentalForeignApi")
            }
        }

        val commonMain by getting {
            dependencies {
                implementation(libs.okio)
                implementation(libs.kotlinx.datetime)
            }
        }

        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }

        val jvmMain by getting {
            dependencies {
                implementation(libs.jna)
            }
        }

        val androidMain by getting {
            dependencies {
                implementation("${libs.jna.get()}@aar")
            }
        }

        val androidInstrumentedTest by getting {
            dependencies {
                implementation(libs.test.junit.ktx)
                implementation(libs.test.runner)
            }
        }
    }
}

android {
    namespace = "technology.breez"
    compileSdk = 33

    defaultConfig {
        minSdk = 21
        consumerProguardFiles("consumer-rules.pro")

        testApplicationId = "technology.breez.test"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

val libraryVersion: String by project

group = "technology.breez"
version = libraryVersion

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
    }

    publications {
        this.forEach {
            (it as MavenPublication).apply {
                pom {
                    name.set("breez-sdk-kmp")
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
}
