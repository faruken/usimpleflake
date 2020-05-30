from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="usimpleflake",
    version="0.0.1",
    author="faruken",
    author_email = "faruken@users.noreply.github.com",
    rust_extensions=[RustExtension("usimpleflake.usimpleflake", binding=Binding.PyO3)],
    zip_safe=False,
)
