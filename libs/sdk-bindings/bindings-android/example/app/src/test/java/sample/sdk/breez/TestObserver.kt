package sample.sdk.breez

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.advanceTimeBy
import org.junit.Assert.assertEquals

class TestObserver<T>(
    scope: CoroutineScope,
    testScheduler: TestCoroutineScheduler,
    flow: Flow<T>
) {
    private val values = mutableListOf<T>()

    @OptIn(ExperimentalCoroutinesApi::class)
    private val job: Job = scope.launch(UnconfinedTestDispatcher(testScheduler)) {
        flow.collect { values.add(it) }
    }

    fun assertValuesAndFinish(vararg values: T): TestObserver<T> {
        assertEquals(values.toList(), this.values)
        job.cancel()
        return this
    }

}

fun <T> Flow<T>.test(
    scope: TestScope
): TestObserver<T> = TestObserver(scope, scope.testScheduler, this)

@OptIn(ExperimentalCoroutinesApi::class)
fun TestScope.advance() = advanceTimeBy(1000L)
