# Wasm Ideas/Notes/Implementation thoughts

 * Vertex enum
   * Data(serde_json::Value)
   * Pass JSON along with other data to plugin
   * Could use Protobufs?
 * Pass in vec of files to plugin
   * Determine if just paths or content as well
   * Allow plugin to read files from system?
 * Plugin config
   * name
   * url/path
   * file types
   * ignored directories
   * data type to be passed
   * type of plugin
     * self contained with Trustfall in plugin
     * adapter passing in value and query details
 * Plugin exposed functions
   * Self-contained
     * Plugin type
     * files, ignored dir etc
     * query
     * lints
   * Adapter
     * Plugin type
     * files, ignored dir etc
     * data type, if JSON, protobuf, etc
     * GraphQL schema
     * trustfall specific functions
 * Tool config
   * plugin configs
   * lints dir/file
   * allow for comparing of lints
   * Determine when lint fails