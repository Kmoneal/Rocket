use std::collections::HashMap;
use std::path::{Path, PathBuf};

use config::{Result, Config, Value, Environment, Limits, LoggingLevel};

/// Structure following the builder pattern for building `Config` structures.
#[derive(Clone)]
pub struct ConfigBuilder {
    /// The environment that this configuration corresponds to.
    pub environment: Environment,
    /// The address to serve on.
    pub address: String,
    /// The port to serve on.
    pub port: u16,
    /// The number of workers to run in parallel.
    pub workers: u16,
    /// Keep-alive timeout in seconds or None if disabled.
    pub keep_alive: Option<u32>,
    /// How much information to log.
    pub log_level: LoggingLevel,
    /// The secret key.
    pub secret_key: Option<String>,
    /// TLS configuration (path to certificates file, path to private key file).
    pub tls: Option<(String, String, Option<String>)>,
    /// Size limits.
    pub limits: Limits,
    /// Any extra parameters that aren't part of Rocket's config.
    pub extras: HashMap<String, Value>,
    /// The root directory of this config.
    pub root: PathBuf,
}

impl ConfigBuilder {
    /// Create a new `ConfigBuilder` instance using the default parameters from
    /// the given `environment`. The root configuration directory is set to the
    /// current working directory.
    ///
    /// This method is typically called indirectly via
    /// [Config::build](/rocket/config/struct.Config.html#method.build).
    ///
    /// # Panics
    ///
    /// Panics if the current directory cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .port(700)
    ///     .workers(12)
    ///     .finalize();
    ///
    /// # assert!(config.is_ok());
    /// ```
    pub fn new(environment: Environment) -> ConfigBuilder {
        let config = Config::new(environment)
            .expect("ConfigBuilder::new(): couldn't get current directory.");

        let root_dir = PathBuf::from(config.root());
        ConfigBuilder {
            environment: config.environment,
            address: config.address,
            port: config.port,
            workers: config.workers,
            keep_alive: config.keep_alive,
            log_level: config.log_level,
            secret_key: None,
            tls: None,
            limits: config.limits,
            extras: config.extras,
            root: root_dir,
        }
    }

    /// Sets the `address` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .unwrap();
    ///
    /// assert_eq!(config.address.as_str(), "127.0.0.1");
    /// ```
    pub fn address<A: Into<String>>(mut self, address: A) -> Self {
        self.address = address.into();
        self
    }

    /// Sets the `port` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .port(1329)
    ///     .unwrap();
    ///
    /// assert_eq!(config.port, 1329);
    /// ```
    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets `workers` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .workers(64)
    ///     .unwrap();
    ///
    /// assert_eq!(config.workers, 64);
    /// ```
    #[inline]
    pub fn workers(mut self, workers: u16) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the keep-alive timeout to `timeout` seconds. If `timeout` is `None`,
    /// keep-alive is disabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .keep_alive(10)
    ///     .unwrap();
    ///
    /// assert_eq!(config.keep_alive, Some(10));
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .keep_alive(None)
    ///     .unwrap();
    ///
    /// assert_eq!(config.keep_alive, None);
    /// ```
    #[inline]
    pub fn keep_alive<T: Into<Option<u32>>>(mut self, timeout: T) -> Self {
        self.keep_alive = timeout.into();
        self
    }

    /// Sets the `log_level` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment, LoggingLevel};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .log_level(LoggingLevel::Critical)
    ///     .unwrap();
    ///
    /// assert_eq!(config.log_level, LoggingLevel::Critical);
    /// ```
    #[inline]
    pub fn log_level(mut self, log_level: LoggingLevel) -> Self {
        self.log_level = log_level;
        self
    }

    /// Sets the `secret_key` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment, LoggingLevel};
    ///
    /// let key = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";
    /// let mut config = Config::build(Environment::Staging)
    ///     .secret_key(key)
    ///     .unwrap();
    /// ```
    pub fn secret_key<K: Into<String>>(mut self, key: K) -> Self {
        self.secret_key = Some(key.into());
        self
    }

    /// Sets the `limits` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment, Limits};
    ///
    /// let mut config = Config::build(Environment::Staging)
    ///     .limits(Limits::new().limit("json", 5 * (1 << 20)))
    ///     .unwrap();
    /// ```
    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    /// Sets the TLS configuration in the configuration being built.
    ///
    /// Certificates are read from `certs_path`. The certificate chain must be
    /// in X.509 PEM format. The private key is read from `key_path`. The
    /// private key must be an RSA key in either PKCS#1 or PKCS#8 PEM format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let cert_store_path: Option<String> = None;
    /// let mut config = Config::build(Environment::Staging)
    ///     .tls("/path/to/certs.pem", "/path/to/key.pem", cert_store_path)
    /// # ; /*
    ///     .unwrap();
    /// # */
    /// ```
    pub fn tls<C, K, W>(mut self, certs_path: C, key_path: K, cert_store_path: W) -> Self
        where C: Into<String>, K: Into<String>, W: Into<Option<String>>
    {
        self.tls = Some((certs_path.into(), key_path.into(), cert_store_path.into()));
        self
    }

    /// Sets the `environment` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .environment(Environment::Production)
    ///     .unwrap();
    ///
    /// assert_eq!(config.environment, Environment::Production);
    /// ```
    #[inline]
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    /// Sets the `root` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .root("/my_app/dir")
    ///     .unwrap();
    ///
    /// assert_eq!(config.root(), Path::new("/my_app/dir"));
    /// ```
    pub fn root<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.root = path.as_ref().to_path_buf();
        self
    }

    /// Adds an extra configuration parameter with `name` and `value` to the
    /// configuration being built. The value can be any type that implements
    /// `Into<Value>` including `&str`, `String`, `Vec<V: Into<Value>>`,
    /// `HashMap<S: Into<String>, V: Into<Value>>`, and most integer and float
    /// types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .extra("pi", 3.14)
    ///     .extra("custom_dir", "/a/b/c")
    ///     .unwrap();
    ///
    /// assert_eq!(config.get_float("pi"), Ok(3.14));
    /// assert_eq!(config.get_str("custom_dir"), Ok("/a/b/c"));
    /// ```
    pub fn extra<V: Into<Value>>(mut self, name: &str, value: V) -> Self {
        self.extras.insert(name.into(), value.into());
        self
    }

    /// Return the `Config` structure that was being built by this builder.
    ///
    /// # Errors
    ///
    /// If the current working directory cannot be retrieved, returns a `BadCWD`
    /// error. If the address or secret key fail to parse, returns a `BadType`
    /// error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .port(700)
    ///     .workers(12)
    ///     .keep_alive(None)
    ///     .finalize();
    ///
    /// assert!(config.is_ok());
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("123.123.123.123.123 whoops!")
    ///     .finalize();
    ///
    /// assert!(config.is_err());
    /// ```
    pub fn finalize(self) -> Result<Config> {
        let mut config = Config::new(self.environment)?;
        config.set_address(self.address)?;
        config.set_port(self.port);
        config.set_workers(self.workers);
        config.set_keep_alive(self.keep_alive);
        config.set_log_level(self.log_level);
        config.set_extras(self.extras);
        config.set_root(self.root);
        config.set_limits(self.limits);

        if let Some((certs_path, key_path, cert_store_path)) = self.tls {
            config.set_tls(&certs_path, &key_path, cert_store_path.as_ref().map(String::as_str))?;
        }

        if let Some(key) = self.secret_key {
            config.set_secret_key(key)?;
        }

        Ok(config)
    }

    /// Return the `Config` structure that was being built by this builder.
    ///
    /// # Panics
    ///
    /// Panics if the current working directory cannot be retrieved or if the
    /// supplied address, secret key, or TLS configuration fail to parse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .unwrap();
    ///
    /// assert_eq!(config.address.as_str(), "127.0.0.1");
    /// ```
    #[inline(always)]
    pub fn unwrap(self) -> Config {
        self.finalize().expect("ConfigBuilder::unwrap() failed")
    }

    /// Returns the `Config` structure that was being built by this builder.
    ///
    /// # Panics
    ///
    /// Panics if the current working directory cannot be retrieved or if the
    /// supplied address, secret key, or TLS configuration fail to parse. If a
    /// panic occurs, the error message `msg` is printed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .expect("the configuration is bad!");
    ///
    /// assert_eq!(config.address.as_str(), "127.0.0.1");
    /// ```
    #[inline(always)]
    pub fn expect(self, msg: &str) -> Config {
        self.finalize().expect(msg)
    }
}
