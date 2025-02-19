import unittest
import asyncio
from callbacks import *

class CallAnswererImpl(CallAnswerer):
    def __init__(self, mode):
        self.mode = mode

    def answer(self):
        if self.mode == "ready":
            return "Bonjour"
        elif self.mode == "busy":
            raise TelephoneError.Busy()
        else:
            raise ValueError("Testing an unexpected error")

class CallbacksTest(unittest.TestCase):
    def test_answer(self):
        cb_object = CallAnswererImpl("ready")
        telephone = Telephone()
        self.assertEqual("Bonjour", telephone.call(cb_object))

    def test_busy(self):
        cb_object = CallAnswererImpl("busy")
        telephone = Telephone()
        with self.assertRaises(TelephoneError.Busy):
            telephone.call(cb_object)

    def test_unexpected_error(self):
        cb_object = CallAnswererImpl("something-else")
        telephone = Telephone()
        with self.assertRaises(TelephoneError.InternalTelephoneError):
            telephone.call(cb_object)

class ProgressReporterImpl(ProgressReporter):
    def __init__(self, on_complete):
        self.on_complete = on_complete

    def report_progress(self, progress):
        print(f"Progress: {progress * 100}%")
        if progress == 1.0:
            self.on_complete()

class ProgressReporterTest(unittest.TestCase):
    def test_threaded_calls(self):
        def run_huge_task_wrapper():
            queue = asyncio.Queue()
            async def get():
                while True:
                    try:
                        queue.get_nowait()
                        return
                    except asyncio.QueueEmpty:
                        await asyncio.sleep(0.1)
            run_huge_task(ProgressReporterImpl(lambda: queue.put_nowait(None)))
            return get()
        asyncio.run(run_huge_task_wrapper())

unittest.main()
