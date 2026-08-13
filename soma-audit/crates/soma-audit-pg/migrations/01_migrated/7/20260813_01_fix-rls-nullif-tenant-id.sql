-- Fix: RLS tenant_isolation policy uses NULLIF to guard against empty-string GUC.
--
-- ROOT CAUSE: SET LOCAL reverts a custom GUC to '' (empty string) at transaction
-- end in PostgreSQL when the GUC was never set at session level. A connection
-- returned to the pool after an audited write carries soma_audit.tenant_id = ''.
-- The original IS NOT NULL guard passes on '' (empty string is not NULL), then
-- ''::uuid raises "invalid input syntax for type uuid: """. This error is
-- intermittent by nature — it only fires on a connection that previously handled
-- an audited write.
--
-- FIX: NULLIF(current_setting(...), '') returns NULL when the GUC is empty or
-- unset. NULL::uuid = NULL, so the row is simply not visible — no error.
-- The redundant IS NOT NULL guard is removed; NULLIF makes it unnecessary.

DROP POLICY IF EXISTS tenant_isolation ON soma_audit.fct_audit_events;
CREATE POLICY tenant_isolation ON soma_audit.fct_audit_events
    USING (tenant_id = NULLIF(current_setting('soma_audit.tenant_id', true), '')::uuid)
    WITH CHECK (tenant_id = NULLIF(current_setting('soma_audit.tenant_id', true), '')::uuid);

-- DOWN ==
DROP POLICY IF EXISTS tenant_isolation ON soma_audit.fct_audit_events;
CREATE POLICY tenant_isolation ON soma_audit.fct_audit_events
    USING (
        current_setting('soma_audit.tenant_id', true) IS NOT NULL
        AND tenant_id = current_setting('soma_audit.tenant_id', true)::uuid
    )
    WITH CHECK (
        current_setting('soma_audit.tenant_id', true) IS NOT NULL
        AND tenant_id = current_setting('soma_audit.tenant_id', true)::uuid
    );
