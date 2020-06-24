package helper

import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class DoubleHelperTest {

    @Test
    fun `almost retrns true for equal doubles`() {
        assertTrue(1.0 approximately 1.0)
    }

    @Test
    fun `almost returns true for doubles within tolerance`() {
        assertTrue(1.0 approximately 1.000009)
    }

    @Test
    fun `almost returns false for doubles outside tolerance`() {
        assertFalse(1.0 approximately 1.00001)
    }
}
