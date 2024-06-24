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
            <td rowspan=3 align="center">Inward</td>
            <td rowspan=1><code>POST</code></td>
            <td rowspan=1 align="center"><code>/api/v1/loans/apply</code></td>
            <td rowspan=1>Submits a loan application for review and approval.</td>
            <td><pre>
  <code>
{
  "loaneeBankAccountNumber": "string",
  "propertyID": "string",
  "loanAmountCents": int,
  "loanDurationMonths": int,
  "downAmountCents": int,
  "creditScore": int,
}
  </code>
</pre></td>
        </tr>
        <tr>
            <td rowspan=1><code>POST</code></td>
            <td rowspan=1 align="center"><code>/api/v1/loans/repayment</code></td>
            <td rowspan=1>Calculates estimated monthly mortgage payments based on loan amount, interest rate, and term.</td>
            <td><pre>
  <code>
{
  "loanAmountCents": int,
  "loanDurationMonths": int,
  "downAmountCents": int,
}
  </code>
</pre></td>
        </tr>
        <tr>
            <td rowspan=1><code>POST</code></td>
            <td rowspan=1 align="center"><code>/api/v1/loans/entity-capacity</code></td>
            <td rowspan=1>Calculates the estimates amount an entity can loan.</td>
            <td><pre>
  <code>
{
  "creditScore": int,
  "liquidWorthCents": int,
  "incomeCents": int,
}
  </code>
</pre></td>
        </tr>
    </tbody>
    
</table>