/**
 * Mods and addons processing
 * support loaders and modpacks
 */
pub mod addons;
/**
 * Archive parser
 * Support zip,gz,7z,jar,json,yml,toml,xml,etc.
 */
pub mod archive;
/**
 * Assets parser and verifier
 */
pub mod assets;
/**
 * Configuration file parser
 */
pub mod config;
/**
 * Downloader
 */
pub mod download;
/**
 * Device Environment
 */
pub mod environment;
/**
 *   Launch Commmand Generators
*/
pub mod generator;
/**
 *   Handlers like crash handler
*/
pub mod handler;
/**
 *  APIs like yggdrasil and ms live oauth
*/
pub mod interface;
/**
 *  Multiplayer Server
*/
pub mod server;
