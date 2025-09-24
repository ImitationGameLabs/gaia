# Frontend Coding Standards

## Code Style

### File Organization
- Component files use PascalCase naming (e.g., `ComponentName.svelte`)
- Utility functions and classes use camelCase naming
- Constants use UPPER_SNAKE_CASE naming

### Component Structure
```svelte
<script>
  // Import statements
  // Type definitions
  // Component logic
</script>

<!-- HTML template -->

<style>
  /* Component styles */
</style>
```

### Naming Conventions
- Variables and functions: `camelCase`
- Components: `PascalCase`
- CSS classes: `kebab-case`
- Event handlers: `handleEventName`

## Best Practices

### Accessibility
- Ensure all interactive elements have appropriate ARIA labels
- Support keyboard navigation
- Provide sufficient color contrast

### Error Handling
- Use try-catch for async operations
- Provide meaningful error messages
- Implement graceful degradation

### Testing
- Write unit tests for critical business logic
- Component tests should cover user interaction scenarios
- Integration tests ensure end-to-end functionality