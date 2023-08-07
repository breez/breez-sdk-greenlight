plugins {
    alias(libs.plugins.androidApp)
    alias(libs.plugins.kotlinAndroid)
    alias(libs.plugins.kotlinKapt)
    alias(libs.plugins.hilt)
}

android {
    namespace = "sample.sdk.breez"
    compileSdk = 33

    defaultConfig {
        applicationId = "sample.sdk.breez"
        minSdk = 21
        targetSdk = 33
        compileSdkVersion = "android-33"
        versionCode = 1
        versionName = "1.0.0"
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.4.8"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }

    testOptions {
        unitTests {
            isReturnDefaultValues = true
        }
    }
}

kapt {
    correctErrorTypes = true
}

dependencies {
    kapt(libs.hiltCompiler)

    implementation(libs.bundles.compose)

    implementation(libs.breez)
    implementation(libs.coreKtx)
    implementation(libs.coroutine)
    implementation(libs.hiltAndroid)
    implementation(libs.hiltNavigationCompose)
    implementation(libs.lifecycleRuntimeKtx)
    implementation(libs.material)
    implementation(libs.navigation)
    implementation(libs.viewModelKtx)

    testImplementation(libs.bundles.unitTest)

    debugImplementation(libs.bundles.composeDebug)
}
