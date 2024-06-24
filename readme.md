# Home Loans Backend

[Economy Graph](https://gremlify.com/uj3behl0kqd/1)


### Real Estate Agent (Sales)
<table>
    <thead>
        <tr>
            <th>Communication Direction</th>
            <th>Endpoint Method</th>
            <th>Endpoint</th>
            <th>Description</th>
            <th>Parameters</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=1 align="center">Outward</td>
        </tr>
        <tr>
            <td rowspan=12 align="center">Inward</td>
            <td rowspan=6><code>POST</code></td>
            <td rowspan=6 align="center"><code>/api/v1/loans/apply</code></td>
            <td rowspan=6>Submits a loan application for review and approval.</td>
            <td><code>personaID</code></td>
        </tr>
        <tr>
            <td><code>propertyID</code></td>
        </tr>
        <tr>
            <td><code>loanAmountCents</code></td>
        </tr>
        <tr>
            <td><code>loanDurationMonths</code></td>
        </tr>
        <tr>
            <td><code>downAmountCents</code></td>
        </tr>
        <tr>
            <td><code>creditScore</code></td>
        </tr>
        <tr>
            <td rowspan=3><code>POST</code></td>
            <td rowspan=3 align="center"><code>/api/v1/loans/repayment</code></td>
            <td rowspan=3>Calculates estimated monthly mortgage payments based on loan amount, interest rate, and term.</td>
            <td><code>loanAmountCents</code></td>
        </tr>
        <tr>
            <td><code>loanDurationMonths</code></td>
        </tr>
        <tr>
            <td><code>downAmountCents</code></td>
        </tr>
        <tr>
            <td rowspan=3><code>POST</code></td>
            <td rowspan=3 align="center"><code>/api/v1/loans/entity-capacity</code></td>
            <td rowspan=3>Calculates the estimates amount an entity can loan.</td>
            <td><code>creditScore</code></td>
        </tr>
        <tr>
            <td><code>liquidWorthCents</code></td>
        </tr>
        <tr>
            <td><code>incomeCents</code></td>
        </tr>
    </tbody>
    
</table>