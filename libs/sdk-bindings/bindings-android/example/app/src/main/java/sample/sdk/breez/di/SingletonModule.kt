package sample.sdk.breez.di

import android.content.Context
import android.content.res.Resources
import breez_sdk.Config
import breez_sdk.EnvironmentType
import breez_sdk.EventListener
import breez_sdk.GreenlightNodeConfig
import breez_sdk.NodeConfig
import breez_sdk.defaultConfig
import breez_sdk.mnemonicToSeed
import dagger.Module
import dagger.Provides
import dagger.Reusable
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import kotlinx.coroutines.Dispatchers
import sample.sdk.breez.EventListenerExample
import technology.breez.BuildConfig
import java.io.File
import javax.inject.Named
import javax.inject.Singleton

@[Module InstallIn(SingletonComponent::class)]
object SingletonModule {

    @[Provides Reusable]
    fun providesResources(
        @ApplicationContext context: Context,
    ): Resources = context.resources

    @[Provides Singleton Named("Seed")]
    fun providesSeed(): List<UByte> = mnemonicToSeed(
        TODO("add your mnemonic phrase here (12 words separated by space)"),
    )

    @[Provides Reusable]
    fun providesEnvironmentType(): EnvironmentType {
        return if (BuildConfig.DEBUG) {
            EnvironmentType.STAGING
        } else {
            EnvironmentType.PRODUCTION
        }
    }

    @[Provides Singleton]
    fun providesGreenlightNodeConfig(): GreenlightNodeConfig = GreenlightNodeConfig(
        null,
        TODO("add your invite code here or null"),
    )

    @[Provides Singleton]
    fun providesNodeConfig(
        greenlightNodeConfig: GreenlightNodeConfig,
    ): NodeConfig = NodeConfig.Greenlight(
        greenlightNodeConfig,
    )

    @[Provides Reusable Named("workingDir")]
    fun providesWorkingDir(
        @ApplicationContext context: Context,
    ): File {
        // You can use any directory you want, here is a trivial sample
        val workingDir = File(context.applicationInfo.dataDir, "sample-working-dir")
        workingDir.mkdirs()
        return workingDir
    }

    @[Provides Singleton]
    fun providesConfig(
        environmentType: EnvironmentType,
        nodeConfig: NodeConfig,
        @Named("workingDir") workingDir: File,
    ): Config {
        val config = defaultConfig(environmentType, TODO("add your api key here"), nodeConfig)
        config.workingDir = workingDir.absolutePath
        return config
    }

    @[Provides Singleton]
    fun providesEventListener(): EventListener = EventListenerExample()

    @[Provides Singleton Named("IO")]
    fun providesIoDispatcher() = Dispatchers.IO

}
