import androidx.test.ext.junit.runners.AndroidJUnit4
import breez_sdk.LogEntry
import breez_sdk.LogStream
import breez_sdk.setLogStream
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class InstrumentedTest {
    @Test
    fun simpleTest() {
        setLogStream(object : LogStream {
            override fun log(l: LogEntry) {
                println(l.line)
            }
        })
    }
}