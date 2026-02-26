# Data Model: Project Framework Update

## Entities

### 1. Project Structure
- **Description**: Represents the directory organization of the monorepo with consistent naming conventions
- **Fields**:
  - `name`: String - The full name of the project structure (e.g., "reusable-library-001-project-framework-core")
  - `root_project_name`: String - The root project name (e.g., "reusable-library")
  - `subproject_number`: Integer - Sequential number for the subproject (e.g., 1)
  - `subproject_type`: String - Type of subproject (e.g., "core", "api", "grpc")
  - `directory_path`: String - Full path to the subproject directory
  - `created_at`: DateTime - When the structure was created
  - `updated_at`: DateTime - When the structure was last updated
- **Relationships**:
  - One-to-many with Subproject entities
- **Validation Rules**:
  - `name` must follow the pattern "rootproject-001-subproject"
  - `root_project_name` must match the repository root project name
  - `subproject_number` must be a positive integer
  - `directory_path` must be a valid file system path

### 2. Subproject
- **Description**: Individual components within the monorepo that follow the naming convention
- **Fields**:
  - `id`: String - Unique identifier for the subproject
  - `name`: String - Full name of the subproject (e.g., "reusable-library-001-project-framework-core")
  - `type`: String - Type of subproject (e.g., "core", "api", "grpc")
  - `description`: String - Brief description of the subproject's purpose
  - `dependencies`: List<String> - List of dependencies required by this subproject
  - `version`: String - Semantic version of the subproject
  - `status`: String - Current status (e.g., "active", "deprecated")
- **Relationships**:
  - Many-to-one with Project Structure
  - Many-to-many with other Subprojects (for dependencies)
- **Validation Rules**:
  - `name` must follow the naming convention pattern
  - `type` must be one of: "core", "api", "grpc"
  - `version` must follow semantic versioning
  - `dependencies` must reference valid subproject names

### 3. Naming Convention
- **Description**: The established pattern for directory naming
- **Fields**:
  - `pattern`: String - The naming convention pattern (e.g., "rootproject-001-subproject")
  - `root_project`: String - The root project name (e.g., "reusable-library")
  - `sequential_number`: Integer - Sequential number for subprojects
  - `subproject_prefix`: String - Prefix for subprojects (e.g., "project-framework")
  - `created_at`: DateTime - When the convention was established
- **Relationships**:
  - One-to-many with Project Structure entities
- **Validation Rules**:
  - `pattern` must be consistent with the constitution requirements
  - `root_project` must match the repository root project name
  - `sequential_number` must be unique within the repository

## State Transitions

### Project Structure Lifecycle
1. **Created** → `name` field populated with valid naming convention
2. **Updated** → `updated_at` timestamp updated when changes occur
3. **Deprecated** → `status` field changed to "deprecated" when subproject is no longer maintained

### Subproject Lifecycle
1. **Created** → Subproject initialized with valid naming convention
2. **Active** → Subproject is actively developed and maintained
3. **Deprecated** → Subproject is marked for deprecation with migration guidance
4. **Removed** → Subproject is completely removed from the repository

## Data Relationships

- A Project Structure can contain multiple Subprojects
- Subprojects can depend on other Subprojects (many-to-many relationship)
- The Naming Convention defines the pattern used by all Project Structures and Subprojects